#![forbid(unsafe_code)]

pub mod common;
pub mod config;
pub mod dto;
pub mod i18n;
pub mod mapper;
pub mod model;
pub mod service;
pub mod util;

use common::error::CrabError;
use config::App;
use rbatis::{db::DBPoolOptions, rbatis::Rbatis};
use redis::{Client, Connection};

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate rbatis;

lazy_static! {
    pub static ref RB: Rbatis = Rbatis::new();
    static ref REDIS: Client = client();
    pub static ref APP: App = App::new();
}

/// 获取 Redis client
fn client() -> Client {
    let client = Client::open(APP.redis_url.clone()).expect("获取 Redis Client 失败");
    client
}

/// 获取 Redis 连接
fn connection() -> Connection {
    let conn = REDIS.get_connection().expect("获取 Redis 连接失败");
    conn
}

/// 初始化数据库
pub async fn init_db() -> Result<(), CrabError> {
    //启用日志输出
    fast_log::init_log("hairy_crabs.log", log::Level::Info, None, true).unwrap();
    //初始化连接池
    let pool_options = DBPoolOptions::default();
    RB.link_opt(APP.database_url.as_str(), pool_options)
        .await
        .map_err(|e| {
            log::error!("数据库连接失败: {}", e);
            CrabError::SQLError("数据库连接失败")
        })?;

    Ok(())
}
