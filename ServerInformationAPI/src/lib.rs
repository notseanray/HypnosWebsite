#![allow(non_snake_case)]
mod config;
use actix_cors::Cors;
use actix_web::{
    get,
    http::{header::ContentType, StatusCode},
    App, HttpRequest, HttpResponse, HttpServer,
};
use anyhow::Result;
use config::*;
use lazy_static::lazy_static;
use notify::{watcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use serenity::{
    async_trait,
    client::Context,
    client::EventHandler,
    futures::StreamExt,
    http::CacheHttp,
    model::{gateway::Ready, guild::Member as SerenityMember, id::ChannelId, prelude::GuildId},
    Client,
};
use std::sync::Arc;
use std::time::Duration;
use tokio::{sync::RwLock, time::sleep};

lazy_static! {
    static ref CONFIG: Arc<RwLock<Config>> = Arc::new(RwLock::new(match Config::load() {
        Ok(v) => v,
        _ => {
            // could just use panic, but this looks better
            println!("invalid config, exiting");
            std::process::exit(1);
        }
    }));
    static ref IMAGES: Arc<RwLock<Vec<String>>> = Arc::new(RwLock::new(Vec::new()));
    static ref MEMBERS: Arc<RwLock<Vec<Member>>> = Arc::new(RwLock::new(Vec::new()));
    static ref SERVERS: Arc<RwLock<Vec<Server>>> = Arc::new(RwLock::new(Vec::new()));
}

#[derive(Default, Deserialize, Serialize)]
pub(crate) struct Member {
    pub avatar: String,
    pub name: String,
}

#[get("/server_status")]
async fn server_status(_: HttpRequest) -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::json())
        .body(serde_json::to_string(&*SERVERS.read().await).unwrap_or_default()))
}

#[get("/discord_members")]
async fn discord_members(_: HttpRequest) -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::json())
        .body(serde_json::to_string(&*MEMBERS.read().await).unwrap_or_default()))
}

#[get("/image_request")]
async fn image_request(_: HttpRequest) -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::json())
        .body(serde_json::to_string(&*IMAGES.read().await).unwrap_or_default()))
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        loop {
            sleep(Duration::from_secs(300)).await;
            let member_role = CONFIG.read().await.member_role_id;
            let id = ChannelId(CONFIG.read().await.showcase_channel_id);
            let get_messages = id.messages(&ctx.http, |r| r.limit(100)).await;
            if let Ok(g) = get_messages {
                let mut urls = Vec::with_capacity(IMAGES.read().await.len());
                IMAGES.write().await.clear();
                for message in &g {
                    for attachment in &message.attachments {
                        urls.push(attachment.url.to_owned());
                    }
                }
                *IMAGES.write().await = urls;
            };
            let guild = GuildId::from(CONFIG.read().await.guild_id);
            let mut members = guild.members_iter(&ctx).boxed();
            // 50 is a decent guess to roughly how many members there are + trial members + bots
            let mut new_members = Vec::with_capacity(50);
            while let Some(m) = members.next().await {
                if let Ok(member) = m {
                    if let Some(r) = member.roles(&ctx.cache().unwrap()).await {
                        if !member.user.bot && r.into_iter().any(|m| m.id.0 == member_role) {
                            new_members.push(member);
                        }
                    }
                }
            }
            MEMBERS.write().await.clear();
            *MEMBERS.write().await = new_members.into_iter().map(|m| m.into()).collect();
        }
    }
}

impl From<SerenityMember> for Member {
    #[inline]
    fn from(m: SerenityMember) -> Self {
        Self {
            avatar: m.user.static_avatar_url().unwrap_or_else(||
                // troll face as default
                "https://media.discordapp.net/stickers/860204185818365962.webp?size=4096"
                .to_owned()),
            name: m.user.name,
        }
    }
}

pub async fn run() -> Result<()> {
    // live reload the config
    tokio::spawn(async move {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = watcher(tx, Duration::from_secs(5)).unwrap();
        watcher.watch(".", RecursiveMode::Recursive).unwrap();
        loop {
            // Send cannot be sent unless the event is dropped, so we must wait until an event
            // happens then reload the config and continue
            while let Ok(notify::DebouncedEvent::Write(_)) = rx.recv() {}
            if let Ok(v) = Config::load() {
                println!("reloaded config");
                // pretty much the only use of anyhow right here, if Box<dyn Error> was Send then I
                // wouldn't use the library
                *CONFIG.write().await = v;
            }
        }
    });

    tokio::spawn(async move {
        loop {
            let servers = &CONFIG.read().await.status;
            // todo, check if you can use rwlock like this
            if let Some(v) = servers {
                let mut new_list = Vec::with_capacity(v.len());
                for server in v {
                    let status = match Server::query(server) {
                        Ok(v) => v,
                        _ => continue,
                    };
                    new_list.push(status);
                }
                SERVERS.write().await.clear();
                *SERVERS.write().await = new_list;
            }
            sleep(Duration::from_secs(30)).await;
        }
    });

    tokio::spawn(async move {
        let mut client = Client::builder(&CONFIG.read().await.token)
            .event_handler(Handler)
            .await
            .unwrap();
        if let Err(e) = client.start().await {
            println!("failed to start Discord client: {e}");
        }
    });

    let port = CONFIG.read().await.port;

    println!("started webserver on 0.0.0.0:{port}");
    HttpServer::new(|| {
        // ideally this shoult not be super permissive, but for this
        // simple rest API I am not very worried
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(server_status)
            .service(discord_members)
            .service(image_request)
    })
    .bind(("0.0.0.0", port))
    .expect("unable to bind to port")
    .workers(1)
    .run()
    .await?;
    Ok(())
}
