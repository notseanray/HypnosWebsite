use anyhow::anyhow;
use anyhow::Result;
use std::time::Duration;
use std::{error::Error, fs, io::Write, path::PathBuf};

use mc_server_ping::ServerStatus;
use mc_server_ping::StatusResponse;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
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
            Some(Duration::from_secs(2)),
            Some(5000000),
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
        Ok(Self {
            display_name: server.display_name.to_owned(),
            online: true,
            player_online: response.players.online,
            player_max: response.players.max,
        })
    }

    fn offline(server: &ConfigServer) -> Self {
        Self {
            display_name: server.display_name.to_owned(),
            online: false,
            player_online: 0,
            player_max: 0,
        }
    }
}

#[derive(Default, Deserialize, Serialize)]
pub(crate) struct ConfigServer {
    host: String,
    port: u16,
    display_name: String,
}

#[derive(Default, Deserialize, Serialize)]
pub(crate) struct Config {
    pub token: String,
    pub port: u16,
    pub guild_id: u64,
    pub member_role_id: u64,
    pub status: Option<Vec<ConfigServer>>,
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
