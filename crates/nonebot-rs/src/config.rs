use colored::*;
use config::Config;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{event, Level};

/// nbrs 配置文件名
pub static CONFIG_PATH: &str = "Nonebotrs.toml";

/// nbrs 配置项结构体
#[derive(Serialize, Deserialize, Clone)]
pub struct NoneBotConfig {
    /// 全局配置
    pub global: GlobalConfig,
    /// bot 配置
    pub bots: Option<HashMap<String, BotConfig>>,
    /// 反向 WS 服务器设置
    pub ws_server: Option<WebSocketServerConfig>,
}

impl std::fmt::Debug for NoneBotConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NoneBotConfig")
            .field("Global", &self.global)
            .field("Bots", &self.bots)
            .finish()
    }
}

/// 反向 WS 服务器设置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WebSocketServerConfig {
    /// Host
    pub host: std::net::Ipv4Addr,
    /// Port
    pub port: u16,
    /// Onebot authorization
    #[serde(alias = "access-token")]
    #[serde(default)]
    access_token: String,
}

/// nbrs 全局配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GlobalConfig {
    /// Debug 模式
    pub debug: bool,
    /// Trace 模式
    pub trace: Option<bool>,
    /// 全局管理员账号设置
    pub superusers: Vec<String>,
    /// 全局昵称设置
    pub nicknames: Vec<String>,
    /// 全局命令起始符设置
    pub command_starts: Vec<String>,
}

/// nbrs bot 配置
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct BotConfig {
    /// bot id
    #[serde(skip)]
    pub bot_id: String,
    /// 管理员账号设置
    #[serde(default)]
    pub superusers: Vec<String>,
    /// 昵称设置
    #[serde(default)]
    pub nicknames: Vec<String>,
    /// 命令起始符设置
    #[serde(default)]
    pub command_starts: Vec<String>,
    #[serde(alias = "access-token")]
    #[serde(default)]
    access_token: String, // Onebot authorization
    /// 正向 WS 地址
    #[serde(default)]
    pub ws_server: String,
}

impl Default for NoneBotConfig {
    fn default() -> Self {
        NoneBotConfig {
            global: GlobalConfig {
                debug: true,
                trace: None,
                superusers: vec![],
                nicknames: vec![],
                command_starts: vec!["/".to_owned()],
            },
            bots: None,
            ws_server: Some(WebSocketServerConfig {
                host: std::net::Ipv4Addr::new(127, 0, 0, 1),
                port: 8088,
                access_token: String::default(),
            }),
        }
    }
}

impl NoneBotConfig {
    /// 从配置文件读取配置
    pub fn load() -> Self {
        use colored::*;
        let config: NoneBotConfig;
        let config_pathbuf = std::path::PathBuf::from(&CONFIG_PATH);
        if !config_pathbuf.exists() {
            config = NoneBotConfig::default();
            let config_string = toml::to_string(&config).unwrap();
            std::fs::write(&config_pathbuf, config_string).unwrap();
            tracing::info!("{}", "未发现配置文件，已新建配置文件。".green());
        } else {
            config = Config::builder()
                .add_source(config::File::with_name(CONFIG_PATH))
                .build()
                .unwrap()
                .try_deserialize::<NoneBotConfig>()
                .unwrap();
        }
        config
    }

    /// 生成 BotConfig
    pub fn gen_bot_config(&self, bot_id: &str) -> BotConfig {
        let mut rbotconfig = BotConfig {
            bot_id: bot_id.to_string(),
            superusers: self.global.superusers.clone(),
            nicknames: self.global.nicknames.clone(),
            command_starts: self.global.command_starts.clone(),
            access_token: String::default(),
            ws_server: String::default(),
        };

        if let Some(server_config) = &self.ws_server {
            rbotconfig.access_token = server_config.access_token.clone();
        }

        if let Some(bots_config) = &self.bots {
            if let Some(bot_config) = bots_config.get(bot_id) {
                if !bot_config.superusers.is_empty() {
                    rbotconfig.superusers = bot_config.superusers.clone();
                }
                if !bot_config.nicknames.is_empty() {
                    rbotconfig.nicknames = bot_config.nicknames.clone();
                }
                if !bot_config.command_starts.is_empty() {
                    rbotconfig.command_starts = bot_config.command_starts.clone();
                }
                if !bot_config.access_token.is_empty() {
                    rbotconfig.access_token = bot_config.access_token.clone();
                }
            }
        }
        rbotconfig
    }

    pub fn gen_access_token(&self) -> AccessToken {
        let mut at = AccessToken {
            global: if let Some(ws_server_config) = &self.ws_server {
                ws_server_config.access_token.clone()
            } else {
                String::default()
            },
            bots: HashMap::default(),
        };
        if let Some(bots) = &self.bots {
            for (bot_id, bot) in bots {
                if !bot.access_token.is_empty() {
                    at.bots
                        .insert(bot_id.to_string(), bot.access_token.to_string());
                }
            }
        }
        at
    }
}

#[derive(Clone)]
pub struct AccessToken {
    pub global: String,
    pub bots: HashMap<String, String>,
}

impl AccessToken {
    pub fn get(&self, bot_id: &str) -> String {
        if let Some(a) = self.bots.get(bot_id) {
            format!("Bearer {}", a)
        } else {
            format!("Bearer {}", &self.global)
        }
    }

    pub fn check_auth(&self, bot_id: &str, token: Option<String>) -> bool {
        let access_token = if let Some(a) = self.bots.get(bot_id) {
            a
        } else {
            &self.global
        };

        if access_token.is_empty() {
            return true;
        }

        fn check(head: &str, token: &str, access_token: &str) -> bool {
            if token.starts_with(head) {
                let token = crate::utils::remove_space(&token.replace(head, ""));
                if token == access_token {
                    return true;
                }
            }
            false
        }

        let mut result = false;
        if let Some(token) = &token {
            result = check("Token", token, access_token) || check("Bearer", token, access_token)
        }

        if !result {
            event!(
                Level::WARN,
                "Access Token match fail Bot:[{}] Token:{:?}",
                bot_id.red(),
                token
            );
        }

        result
    }
}
