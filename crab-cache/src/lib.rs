//! 缓存

use crab_config::APP;
use crab_lib::{
    lazy_static::lazy_static,
    redis::{Client, Connection},
};

pub mod config_util;
pub mod redis_cache;

/// 缓存接口
pub trait CacheUtil {
    /// 获取缓存键
    fn cache_key() -> &'static str;
    /// 缓存信息
    fn cache_value(key: &str) -> Option<String>;
    /// 备注信息
    fn remark() -> &'static str;
    /// 清除缓存信息
    fn clear_cache();
    /// 清除缓存信息
    fn clear_cache_by_keys(keys: &[&str]);
    /// 获取缓存名
    fn cache_keys() -> Vec<String>;
}

lazy_static! {
    static ref REDIS: Client = client();
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
