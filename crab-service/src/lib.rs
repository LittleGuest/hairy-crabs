#![forbid(unsafe_code)]

use config::ConfigSrv;
use crab_lib::lazy_static::lazy_static;
use dict::{DictDataSrv, DictTypeSrv};
use gen::{GenConfigTemplateSrv, GenTableColumnSrv, GenTableSrv};
use log::{LogSrv, LoginLogSrv};
use menu::MenuSrv;
use role::RoleSrv;
use user::{SysLoginSrv, UserRoleSrv, UserSrv};

mod config;
mod dict;
mod gen;
mod log;
mod menu;
mod role;
mod user;

/// 服务
pub struct Service {
    pub login: SysLoginSrv,
    pub user: UserSrv,
    pub config: ConfigSrv,
    pub dict_type: DictTypeSrv,
    pub dict_data: DictDataSrv,
    pub menu: MenuSrv,
    pub role: RoleSrv,
    pub user_role: UserRoleSrv,
    pub log: LogSrv,
    pub login_log: LoginLogSrv,
    pub gen_config_template: GenConfigTemplateSrv,
    pub gen_table: GenTableSrv,
    pub gen_table_column: GenTableColumnSrv,
}

impl Service {
    fn new() -> Self {
        Self {
            login: SysLoginSrv,
            user: UserSrv,
            config: ConfigSrv,
            dict_type: DictTypeSrv,
            dict_data: DictDataSrv,
            menu: MenuSrv,
            role: RoleSrv,
            user_role: UserRoleSrv,
            log: LogSrv,
            login_log: LoginLogSrv,
            gen_config_template: GenConfigTemplateSrv,
            gen_table: GenTableSrv,
            gen_table_column: GenTableColumnSrv,
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
