#![forbid(unsafe_code)]

use config::ConfigSrv;
use crab_lib::lazy_static::lazy_static;
use dict::{DictDataSrv, DictTypeSrv};
use log::{LogSrv, LoginLogSrv};
use menu::MenuSrv;
use role::RoleSrv;
use tool_gen::GenSrv;
use user::{SysLoginSrv, UserRoleSrv, UserSrv};

mod config;
mod dict;
mod log;
mod menu;
mod role;
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
    pub role: RoleSrv,
    pub user_role: UserRoleSrv,
    pub log: LogSrv,
    pub login_log: LoginLogSrv,
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
            role: RoleSrv,
            user_role: UserRoleSrv,
            log: LogSrv,
            login_log: LoginLogSrv,
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
