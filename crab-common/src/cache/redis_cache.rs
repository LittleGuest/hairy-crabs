//! Redis 相关

use std::sync::{Arc, Mutex};

use redis::{Client, Commands, Connection, FromRedisValue, ToRedisArgs};

const HOSTNAME: &str = "127.0.0.1";
const USERNAME: &str = "";
const PASSWORD: &str = "";
const PORT: u16 = 6379;
const DB: u8 = 0;

fn redis_ur() -> String {
    if USERNAME.is_empty() && PASSWORD.is_empty() {
        format!("redis://{}:{}/{}", HOSTNAME, PORT, DB)
    } else {
        format!(
            "redis://{}:{}@{}:{}/{}",
            USERNAME, PASSWORD, HOSTNAME, PORT, DB
        )
    }
}

lazy_static! {
    // static ref REDIS: Arc<Mutex<Connection>> = Arc::new(Mutex::new(connection()));
    static ref REDIS: Client = client();
}

fn client() -> Client {
    let client = Client::open(redis_ur()).expect("获取 Redis Client 失败");
    client
}

fn connection() -> Connection {
    let mut conn = REDIS.get_connection().expect("获取 Redis 连接失败");
    conn
}

pub fn set(k: &str, v: &impl ToRedisArgs) {
    let _: () = redis::cmd("SET")
        .arg(k)
        .arg(v)
        .query(&mut connection())
        .unwrap();
}

pub fn get<T: FromRedisValue>(k: &str) -> Option<T> {
    let conn = &mut connection();

    match redis::cmd("GET").arg(k).query::<T>(&mut connection()) {
        Ok(v) => Some(v),
        Err(_) => None,
    }
}

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
