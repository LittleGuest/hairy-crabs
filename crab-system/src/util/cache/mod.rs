//! 缓存工具

pub mod config_util;

/// 缓存接口
pub trait CacheUtil {
    /// 获取键
    fn cache_key() -> &'static str;
    ///
    #[deprecated]
    fn cache_id() -> &'static str;
    /// 根据键获取值
    fn cache_value(key: &str) -> Option<String>;
    /// 备注信息
    fn remark() -> &'static str;
    /// 清除缓存
    fn clear_cache();
    /// Hash 结构： 清除指定键缓存
    fn clear_cache_by_keys(keys: &[&str]);
    /// Hash 结构：获取所有的键
    fn cache_keys() -> Option<Vec<String>>;
}
