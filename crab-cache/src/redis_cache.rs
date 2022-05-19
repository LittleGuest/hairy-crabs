use std::str::FromStr;

use crab_lib::{log, redis::types::RedisArg};

use crate::REDIS;

pub struct RedisCache;

impl RedisCache {
    pub fn get<T: FromStr>(k: &str) -> Option<T> {
        // REDIS.lock().unwrap().get::<T>(k).map_or(None,|t|Some(t));
        match REDIS.lock().unwrap().get::<T>(k) {
            Ok(t) => Some(t),
            Err(e) => {
                log::error!("RedisCache::get error: {e}");
                None
            }
        }
    }

    pub fn get_string(k: &str) -> Option<String> {
        Self::get::<String>(k)
    }

    pub fn set<T: RedisArg>(k: &str, v: T) {
        if let Err(e) = REDIS.lock().unwrap().set(k, v) {
            log::error!("RedisCache::set error: {e}");
        }
    }

    pub fn setex<T: RedisArg>(k: &str, v: T, seconds: usize) {
        if let Err(e) = REDIS.lock().unwrap().setex(k, v, seconds) {
            log::error!("RedisCache::setex error: {e}");
        }
    }

    pub fn setnx<T: RedisArg>(k: &str, v: T) {
        if let Err(e) = REDIS.lock().unwrap().setnx(k, v) {
            log::error!("RedisCache::setnx error: {e}");
        }
    }

    pub fn getset<T: RedisArg, V: FromStr>(k: &str, v: T) -> V {
        REDIS.lock().unwrap().getset(k, v).unwrap()
    }

    pub fn getset_string<T: RedisArg>(k: &str, v: T) -> Option<String> {
        match REDIS.lock().unwrap().getset_string(k, v) {
            Ok(t) => Some(t),
            Err(e) => {
                log::error!("RedisCache::getset_string error: {e}");
                None
            }
        }
    }

    pub fn del(k: &str) {
        if let Err(e) = REDIS.lock().unwrap().del(k) {
            log::error!("RedisCache::del error: {e}");
        }
    }

    pub fn exists(k: &str) -> bool {
        match REDIS.lock().unwrap().exists(k) {
            Ok(t) => t,
            Err(e) => {
                log::error!("RedisCache::exists error: {e}");
                false
            }
        }
    }

    pub fn expire(k: &str, seconds: usize) {
        if let Err(e) = REDIS.lock().unwrap().expire(k, seconds) {
            log::error!("RedisCache::expire error: {e}");
        }
    }

    // pub fn pexpire(&mut self, key: &str, millies: usize) -> RedisEmptyResult {
    //     self.run_command_empty_response("PEXPIRE", vec![key, &*millies.to_string()])
    // }

    // pub fn persist(&mut self, key: &str) -> RedisEmptyResult {
    //     self.run_command_empty_response("PERSIST", vec![key])
    // }

    // pub fn rename(&mut self, key: &str, new_key: &str) -> RedisEmptyResult {
    //     self.run_command_empty_response("RENAME", vec![key, new_key])
    // }

    // pub fn renamenx(&mut self, key: &str, new_key: &str) -> RedisEmptyResult {
    //     self.run_command_empty_response("RENAMENX", vec![key, new_key])
    // }

    // pub fn append(&mut self, key: &str, value: &str) -> RedisEmptyResult {
    //     self.run_command_empty_response("APPEND", vec![key, value])
    // }

    // pub fn incr(&mut self, key: &str) -> RedisResult<i64> {
    //     self.run_command::<i64>("INCR", vec![key])
    // }

    // pub fn incrby<T: RedisArg>(&mut self, key: &str, value: T) -> RedisResult<i64> {
    //     self.run_command::<i64>("INCRBY", vec![key, &*value.to_string()])
    // }

    // pub fn incrbyfloat<T: RedisArg>(&mut self, key: &str, value: T) -> RedisResult<f64> {
    //     self.run_command::<f64>("INCRBYFLOAT", vec![key, &*value.to_string()])
    // }

    // pub fn strlen(&mut self, key: &str) -> RedisResult<i32> {
    //     self.run_command::<i32>("STRLEN", vec![key])
    // }

    // pub fn keys(&mut self, pattern: &str) -> RedisResult<Vec<String>> {
    //     self.run_command::<Vec<String>>("KEYS", vec![pattern])
    // }

    pub fn hget<T: FromStr>(k: &str, field: &str) -> Option<T> {
        match REDIS.lock().unwrap().hget(k, field) {
            Ok(t) => Some(t),
            Err(e) => {
                log::error!("RedisCache::hget error: {e}");
                None
            }
        }
    }

    // pub fn hget_string(self: &mut Client, key: &str, field: &str) -> RedisStringResult {
    //     self.run_command_string_response("HGET", vec![key, field])
    // }

    // ///
    // /// # Example
    // ///
    // /// ```
    // /// # let mut client = simple_redis::create("redis://127.0.0.1:6379/").unwrap();
    // /// match client.hgetall("my_map") {
    // ///     Ok(map) => {
    // ///         match map.get("my_field") {
    // ///             Some(value) => println!("Got field value from map: {}", value),
    // ///             None => println!("Map field is emtpy"),
    // ///         }
    // ///     },
    // ///     Err(error) => println!("Unable to read map from Redis: {}", error),
    // /// }
    // /// ```
    // ///
    // pub fn hgetall(self: &mut Client, key: &str) -> RedisResult<HashMap<String, String>> {
    //     self.run_command::<HashMap<String, String>>("HGETALL", vec![key])
    // }

    // pub fn hset<T: RedisArg>(
    //     self: &mut Client,
    //     key: &str,
    //     field: &str,
    //     value: T,
    // ) -> RedisEmptyResult {
    //     self.run_command_empty_response("HSET", vec![key, field, &value.to_string()])
    // }

    // pub fn hsetnx<T: RedisArg>(
    //     self: &mut Client,
    //     key: &str,
    //     field: &str,
    //     value: T,
    // ) -> RedisEmptyResult {
    //     self.run_command_empty_response("HSETNX", vec![key, field, &value.to_string()])
    // }

    // pub fn hdel(self: &mut Client, key: &str, field: &str) -> RedisEmptyResult {
    //     self.run_command_empty_response("HDEL", vec![key, field])
    // }

    // pub fn hexists(self: &mut Client, key: &str, field: &str) -> RedisBoolResult {
    //     self.run_command_bool_response("HEXISTS", vec![key, field])
    // }

    pub fn hkeys(k: &str) -> Vec<String> {
        match REDIS.lock().unwrap().hkeys(k) {
            Ok(t) => t,
            Err(e) => {
                log::error!("RedisCache::hget error: {e}");
                vec![]
            }
        }
    }

    // pub fn hvals(&mut self, key: &str) -> RedisResult<Vec<String>> {
    //     self.run_command::<Vec<String>>("HVALS", vec![key])
    // }

    // pub fn lset<T: RedisArg>(
    //     self: &mut Client,
    //     key: &str,
    //     index: isize,
    //     value: T,
    // ) -> RedisEmptyResult {
    //     self.run_command_empty_response("LSET", vec![key, &index.to_string(), &value.to_string()])
    // }

    // pub fn lindex<T: FromStr>(self: &mut Client, key: &str, index: isize) -> RedisResult<T> {
    //     self.run_command_from_string_response("LINDEX", vec![key, &index.to_string()])
    // }

    // pub fn lindex_string(self: &mut Client, key: &str, index: isize) -> RedisStringResult {
    //     self.run_command_string_response("LINDEX", vec![key, &index.to_string()])
    // }

    // pub fn llen(self: &mut Client, key: &str) -> RedisResult<i32> {
    //     self.run_command::<i32>("LLEN", vec![key])
    // }

    // pub fn lpop<T: FromStr>(self: &mut Client, key: &str) -> RedisResult<T> {
    //     self.run_command_from_string_response("LPOP", vec![key])
    // }

    // pub fn lpush<T: RedisArg>(self: &mut Client, key: &str, value: T) -> RedisEmptyResult {
    //     self.run_command_empty_response("LPUSH", vec![key, &value.to_string()])
    // }

    // pub fn lpushx<T: RedisArg>(self: &mut Client, key: &str, value: T) -> RedisEmptyResult {
    //     self.run_command_empty_response("LPUSHX", vec![key, &value.to_string()])
    // }

    // pub fn lrange(
    //     self: &mut Client,
    //     key: &str,
    //     start: isize,
    //     stop: isize,
    // ) -> RedisResult<Vec<String>> {
    //     self.run_command::<Vec<String>>("LRANGE", vec![key, &start.to_string(), &stop.to_string()])
    // }

    // pub fn lrem<T: RedisArg>(
    //     self: &mut Client,
    //     key: &str,
    //     count: isize,
    //     value: T,
    // ) -> RedisEmptyResult {
    //     self.run_command_empty_response("LREM", vec![key, &count.to_string(), &value.to_string()])
    // }

    // pub fn ltrim(self: &mut Client, key: &str, start: isize, stop: isize) -> RedisEmptyResult {
    //     self.run_command_empty_response("LTRIM", vec![key, &start.to_string(), &stop.to_string()])
    // }

    // pub fn rpop<T: FromStr>(self: &mut Client, key: &str) -> RedisResult<T> {
    //     self.run_command_from_string_response("RPOP", vec![key])
    // }

    // pub fn rpush<T: RedisArg>(self: &mut Client, key: &str, value: T) -> RedisEmptyResult {
    //     self.run_command_empty_response("RPUSH", vec![key, &value.to_string()])
    // }

    // pub fn rpushx<T: RedisArg>(self: &mut Client, key: &str, value: T) -> RedisEmptyResult {
    //     self.run_command_empty_response("RPUSHX", vec![key, &value.to_string()])
    // }

    // pub fn sadd(self: &mut Client, key: &str, member: &str) -> RedisResult<i32> {
    //     self.run_command::<i32>("SADD", vec![key, member])
    // }

    // pub fn scard(self: &mut Client, key: &str) -> RedisResult<i32> {
    //     self.run_command::<i32>("SCARD", vec![key])
    // }

    // pub fn sdiff(self: &mut Client, keys: Vec<&str>) -> RedisResult<Vec<String>> {
    //     self.run_command::<Vec<String>>("SDIFF", keys)
    // }

    // pub fn sismember(self: &mut Client, key: &str, member: &str) -> RedisBoolResult {
    //     self.run_command("SISMEMBER", vec![key, member])
    // }

    // pub fn smembers(self: &mut Client, key: &str) -> RedisResult<Vec<String>> {
    //     self.run_command::<Vec<String>>("SMEMBERS", vec![key])
    // }

    // pub fn smove(
    //     self: &mut Client,
    //     source_key: &str,
    //     destination_key: &str,
    //     member: &str,
    // ) -> RedisEmptyResult {
    //     self.run_command("SMOVE", vec![source_key, destination_key, member])
    // }

    // pub fn srem(self: &mut Client, key: &str, member: &str) -> RedisEmptyResult {
    //     self.run_command("SREM", vec![key, member])
    // }

    // pub fn zadd(self: &mut Client, key: &str, score: isize, member: &str) -> RedisResult<i32> {
    //     self.run_command("ZADD", vec![key, &score.to_string(), member])
    // }

    // pub fn zrange(
    //     self: &mut Client,
    //     key: &str,
    //     start: isize,
    //     stop: isize,
    // ) -> RedisResult<Vec<String>> {
    //     self.run_command::<Vec<String>>("ZRANGE", vec![key, &start.to_string(), &stop.to_string()])
    // }
}
