use anyhow::Result;
use std::time::Duration;
use std::{fs, io::Write, path::PathBuf};

use mc_server_ping::ServerStatus;
use serde::{Deserialize, Serialize};

const REQUEST_TIMEOUT: u64 = 5000;
// 10 MB
const MAX_REQUEST_SIZE: u32 = 1048576 * 10;

#[derive(Default, Deserialize, Serialize, Debug)]
pub(crate) struct Server {
    display_name: String,
    online: bool,
    player_online: i64,
    player_max: i64,
}

impl Server {
    pub(crate) fn query(server: &ConfigServer) -> Result<Self, std::io::Error> {
        // 5mb 2 second max
        let mut query = ServerStatus::new(
            &server.host,
            server.port,
            Some(Duration::from_millis(REQUEST_TIMEOUT)),
            Some(MAX_REQUEST_SIZE),
        );
        // if I used proc macros I could rewrite this to be a macro, but oh well
        match query.query() {
            Ok(v) => v,
            _ => return Ok(Self::offline(server)),
        };
        let response = match query.to_json() {
            Ok(v) => v,
            _ => return Ok(Self::offline(server)),
        };
        let mut player_online = 0;
        let mut player_max = 0;
        if let Some(v) = response.players {
            player_online = v.online;
            player_max = v.max;
        }
        Ok(Self {
            display_name: server.display_name.to_owned(),
            online: true,
            player_online,
            player_max,
        })
    }

    #[inline]
    fn offline(server: &ConfigServer) -> Self {
        Self {
            display_name: server.display_name.to_owned(),
            online: false,
            player_online: 0,
            player_max: 0,
        }
    }
}

#[derive(Default, Deserialize, Serialize, Debug)]
pub(crate) struct ConfigServer {
    pub host: String,
    pub port: u16,
    pub display_name: String,
}

#[derive(Default, Deserialize, Serialize, Debug)]
pub(crate) struct Config {
    pub token: String,
    pub port: u16,
    pub guild_id: u64,
    pub member_role_id: u64,
    pub status: Option<Vec<ConfigServer>>,
    pub showcase_channel_id: u64,
}

impl Config {
    pub fn load() -> Result<Self> {
        let data = match fs::read_to_string("config.json") {
            Ok(v) => v,
            _ => {
                println!("failed to load config file");
                if !PathBuf::from("config.json").exists() {
                    println!("generating blank config");
                    let mut new_config = fs::File::create("config.json")?;
                    new_config
                        .write_all(serde_json::to_string_pretty(&Config::default())?.as_bytes())?;
                } else {
                    println!("failed to read config file, is file correct?");
                }
                std::process::exit(1);
            }
        };
        Ok(serde_json::from_str(&data)?)
    }
}
