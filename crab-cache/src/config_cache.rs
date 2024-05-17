use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crab_lib::{async_static::async_static, log};
use crab_model::{Mapper, SysConfig};

async fn init_config_cache() -> Arc<Mutex<HashMap<String, SysConfig>>> {
    match SysConfig::list().await {
        Ok(scs) => {
            let len = scs.len();
            let cache = scs
                .into_iter()
                .fold(HashMap::with_capacity(len), |mut cache, sc| {
                    cache.insert(sc.config_key.clone().unwrap_or("".to_string()), sc);
                    cache
                });
            Arc::new(Mutex::new(cache))
        }
        Err(e) => {
            log::error!("初始化系统参数配置缓存错误：{}", e);
            Arc::new(Mutex::new(HashMap::with_capacity(0)))
        }
    }
}

async_static! {
    pub   static  ref ConfigCache: Arc<Mutex<HashMap<String, SysConfig>>> = init_config_cache().await;
}

impl ConfigCache {
    pub async fn set(k: &str, v: SysConfig) {
        ConfigCache.await.lock().unwrap().insert(k.to_string(), v);
    }

    pub async fn get(k: &str) -> Option<&SysConfig> {
        let cc = ConfigCache.await.lock().expect("msg");
        cc.get(k)
    }

    pub async fn values() -> Vec<SysConfig> {
        ConfigCache
            .await
            .lock()
            .unwrap()
            .values()
            .cloned()
            .collect::<Vec<_>>()
    }
}
