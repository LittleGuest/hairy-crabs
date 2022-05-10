#![forbid(unsafe_code)]

use crab_common::error::CrabError;
use crab_config::APP;
use crab_lib::lazy_static::lazy_static;
use fast_log::{
    config::Config,
    consts::LogSize,
    plugin::{file_split::RollingType, packer::LogPacker},
};
use rbatis::{db::DBPoolOptions, rbatis::Rbatis};

mod sys_user;
pub use sys_user::*;
mod sys_menu;
pub use sys_menu::*;
mod gen_config_template;
pub use gen_config_template::*;
mod gen_table;
pub use gen_table::*;
mod gen_table_column;
pub use gen_table_column::*;
mod dto;
pub use dto::*;

pub mod mapper;

lazy_static! {
    pub static ref RB: Rbatis = Rbatis::new();
}

/// 初始化数据库
pub async fn init_db() -> Result<(), CrabError> {
    // 启用日志输出
    fast_log::init(Config::new().console().file_split(
        "target/logs/",
        LogSize::MB(1),
        RollingType::All,
        LogPacker {},
    ))
    .unwrap();

    //初始化连接池
    log::info!("初始化数据库连接");
    let pool_options = DBPoolOptions::default();
    RB.link_opt(APP.database_url.as_str(), pool_options)
        .await
        .map_err(|_e| {
            // log::error!("数据库连接失败: {}", e);
            CrabError::ServerError("数据库连接失败")
        })?;
    log::info!("初始化数据库连接完成");

    Ok(())
}
