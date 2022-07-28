#![allow(non_snake_case)]
mod config;
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
    http::CacheHttp,
    model::{event::GatewayEvent, gateway::Ready},
    prelude::GatewayIntents,
    Client,
};
use std::sync::Arc;
use std::{sync::atomic::AtomicBool, time::Duration};
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
    static ref MEMBERS: Arc<RwLock<Vec<Member>>> = Arc::new(RwLock::new(vec![]));
    static ref SERVERS: Arc<RwLock<Vec<Server>>> = Arc::new(RwLock::new(vec![]));
}

#[derive(Default, Deserialize, Serialize)]
pub(crate) struct Member {
    pub avatar: String,
    pub name: String,
}

#[get("/server_status")]
async fn server_status(_: HttpRequest) -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::plaintext())
        .body(serde_json::to_string(&*SERVERS.read().await).unwrap_or_default()))
}

#[get("/discord_members")]
async fn discord_members(_: HttpRequest) -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::plaintext())
        .body(serde_json::to_string(&*MEMBERS.read().await).unwrap_or_default()))
}

#[get("/image_request")]
async fn image_request(_: HttpRequest) -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::plaintext())
        .body("test".to_string()))
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        loop {
            sleep(Duration::from_secs(30)).await;
            let member_role = CONFIG.read().await.member_role_id;
            let guild = ctx.cache.guild(CONFIG.read().await.guild_id);
            if let Some(v) = guild {
                let members = match v.members(ctx.http(), None, None).await {
                    Ok(v) => v.into_iter().filter(|m| {
                        m.roles(ctx.cache().unwrap())
                            .unwrap()
                            .into_iter()
                            .any(|x| x.id.0 == member_role && !m.user.bot)
                    }),
                    _ => continue,
                };
                let mut updated_members: Vec<Member> =
                    Vec::with_capacity(MEMBERS.read().await.len());
                MEMBERS.write().await.clear();
                for member in members {
                    updated_members.push(Member { avatar: member.avatar_url().unwrap_or_else(|| "https://media.discordapp.net/stickers/860204185818365962.webp?size=4096".to_owned()), name: member.user.name })
                }
                *MEMBERS.write().await = updated_members;
            }
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
                    SERVERS.write().await.clear();
                    if let Ok(v) = Server::query(server) {
                        new_list.push(v);
                    }
                }
            }
            sleep(Duration::from_secs(30)).await;
        }
    });

    tokio::spawn(async move {
        let intents = GatewayIntents::GUILD_MEMBERS | GatewayIntents::GUILD_BANS;
        let mut client = Client::builder(&CONFIG.read().await.token, intents)
            .event_handler(Handler)
            .await
            .unwrap();
        if let Err(e) = client.start().await {
            println!("failed to start Discord client: {e}");
        }
    });

    HttpServer::new(|| App::new().service(server_status))
        .bind(("0.0.0.0", CONFIG.read().await.port))
        .expect("unable to bind to port")
        .workers(1)
        .run()
        .await?;
    Ok(())
}
