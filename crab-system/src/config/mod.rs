//! 应用配置

use serde::{Deserialize, Serialize};

/// 服务信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    // 服务名称
    pub name: Option<String>,
    // 服务地址
    pub host: String,
    // 服务端口
    pub port: u16,
}

/// 逻辑删除字段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Logic {
    // 列名
    pub column: String,
    // 正常
    pub normal: u8,
    // 已删除
    pub deleted: u8,
}

/// 应用
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App {
    pub server: Server,
    // 数据库地址
    pub database_url: String,
    pub logic: Option<Logic>,
    // redis地址
    pub redis_url: String,
    // 日志目录
    pub log_dir: String,
    // JWT 密钥
    pub jwt_secret: String,
    // 白名单
    pub white_list: Vec<String>,
}

impl std::fmt::Display for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", json!(self))
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            server: Server {
                name: Some("Hairy-Crab".to_string()),
                host: "127.0.0.1".to_string(),
                port: 8080,
            },
            database_url: Default::default(),
            logic: Default::default(),
            redis_url: Default::default(),
            log_dir: Default::default(),
            jwt_secret: Default::default(),
            white_list: vec![],
        }
    }
}

impl App {
    pub fn new() -> Self {
        let setting = include_str!("../../../crab.yml");
        println!("{}", setting);
        serde_yaml::from_str::<App>(setting).expect("配置文件错误")
    }
}
