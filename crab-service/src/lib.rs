#![forbid(unsafe_code)]

use crab_lib::lazy_static::lazy_static;
use tool_gen::GenSrv;
use user::{SysLoginSrv, UserSrv};

mod tool_gen;
mod user;

/// 服务
pub struct Service {
    pub login: SysLoginSrv,
    pub user: UserSrv,
    pub gen: GenSrv,
}

impl Service {
    fn new() -> Self {
        Self {
            login: SysLoginSrv,
            user: UserSrv,
            gen: GenSrv,
        }
    }
}

impl Default for Service {
    fn default() -> Self {
        Self::new()
    }
}

lazy_static! {
    pub static ref SRV: Service = Service::new();
}
