#![forbid(unsafe_code)]

use config::ConfigSrv;
use crab_lib::lazy_static::lazy_static;
use dict::{DictDataSrv, DictTypeSrv};
use menu::MenuSrv;
use tool_gen::GenSrv;
use user::{SysLoginSrv, UserSrv};

mod config;
mod dict;
mod menu;
mod tool_gen;
mod user;

/// 服务
pub struct Service {
    pub login: SysLoginSrv,
    pub user: UserSrv,
    pub gen: GenSrv,
    pub config: ConfigSrv,
    pub dict_type: DictTypeSrv,
    pub dict_data: DictDataSrv,
    pub menu: MenuSrv,
}

impl Service {
    fn new() -> Self {
        Self {
            login: SysLoginSrv,
            user: UserSrv,
            gen: GenSrv,
            config: ConfigSrv,
            dict_type: DictTypeSrv,
            dict_data: DictDataSrv,
            menu: MenuSrv,
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
