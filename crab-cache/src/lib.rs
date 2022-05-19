//! 缓存

use std::sync::{Arc, Mutex};

use crab_config::APP;
use crab_lib::{
    lazy_static::lazy_static,
    redis::{self, client::Client},
};

mod config_util;
pub use config_util::ConfigUtil;

mod redis_cache;
pub use redis_cache::RedisCache;

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
    static ref REDIS: Arc<Mutex<Client>> = client();
}

fn client() -> Arc<Mutex<Client>> {
    let client = redis::create(APP.redis_url.as_str()).expect("获取 Redis Client 失败");
    Arc::new(Mutex::new(client))
}
