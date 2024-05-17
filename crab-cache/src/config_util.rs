use super::CacheUtil;
use crate::redis_cache::RedisCache;

pub struct ConfigUtil;

impl CacheUtil for ConfigUtil {
    fn cache_key() -> &'static str {
        Self::CACHE_KEY
    }

    fn cache_value(key: &str) -> Option<String> {
        RedisCache::hget::<String>(Self::CACHE_KEY, key)
    }

    fn remark() -> &'static str {
        "配置信息"
    }

    fn clear_cache() {
        RedisCache::del(Self::cache_key());
    }

    fn clear_cache_by_keys(_keys: &[&str]) {}

    fn cache_keys() -> Vec<String> {
        RedisCache::hkeys(Self::CACHE_KEY)
    }
}

impl ConfigUtil {
    /// 键名
    const CACHE_KEY: &'static str = "configCache";

    // pub fn get_cache_key(config_key: &str) -> String {
    //     format!("{}{}", consts::SYS_CONFIG_KEY, config_key)
    // }

    // pub fn get_config_by_key(config_key: &str) -> SysConfig {
    //     match redis_cache::h_get::<SysConfig>(
    //         Self::CACHE_KEY,
    //         Self::get_cache_key(config_key).as_str(),
    //     ) {
    //         Some(_sc) => todo!(),
    //         None => todo!(),
    //     }
    // }

    /// 根据 key 值获取配置的 bool 值
    pub fn get_config_bool_value_by_key(k: &str, default: bool) -> bool {
        RedisCache::get::<bool>(k).map_or(default, |v| v)
    }
}
