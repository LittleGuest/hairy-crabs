//! Redis 相关

use crab_lib::redis::{self, FromRedisValue, ToRedisArgs};

use crate::connection;

/// 字符串 设置
pub fn set(key: &str, v: &impl ToRedisArgs) {
    let _: () = redis::cmd("SET")
        .arg(key)
        .arg(v)
        .query(&mut connection())
        .unwrap();
}

/// 字符串获取
pub fn get<T>(key: &str) -> Option<T>
where
    T: FromRedisValue,
{
    let conn = &mut connection();

    match redis::cmd("GET").arg(key).query::<T>(conn) {
        Ok(v) => Some(v),
        Err(_) => None,
    }
}

/// Hash：根据 key 和 field 获取
pub fn h_get<T>(key: &str, field: &str) -> Option<T>
where
    T: FromRedisValue,
{
    match redis::cmd("HGET")
        .arg(key)
        .arg(field)
        .query::<T>(&mut connection())
    {
        Ok(v) => Some(v),
        Err(_) => None,
    }
}

/// Hash ：获取所有的键
pub fn h_keys(key: &str) -> Option<Vec<String>> {
    match redis::cmd("HKEYS")
        .arg(key)
        .query::<Vec<String>>(&mut connection())
    {
        Ok(v) => Some(v),
        Err(_) => None,
    }
}

/// 根据键删除单个
pub fn del(k: &str) {
    redis::cmd("DEL").arg(k).execute(&mut connection());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_redis() {
        let key = "crab_test_get";
        let value = "crab_test_get_value".to_string();

        assert_eq!(None, get::<String>(key));
        set(key, &value);
        assert_eq!(Some(value), get::<String>(key));
        del(key);
    }
}
