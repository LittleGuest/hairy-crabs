#![forbid(unsafe_code)]

/// 初始化数据库
// pub async fn init_db() -> Result<(), CrabError> {
//     //启用日志输出
//     fast_log::init_log("hairy_crabs.log", log::Level::Info, None, true).unwrap();
//     //初始化连接池
//     let pool_options = DBPoolOptions::default();
//     RB.link_opt(APP.database_url.as_str(), pool_options)
//         .await
//         .map_err(|e| {
//             log::error!("数据库连接失败: {}", e);
//             CrabError::SQLError("数据库连接失败")
//         })?;

//     Ok(())
// }
use crab_lib::lazy_static::lazy_static;
use rbatis::rbatis::Rbatis;

mod sys_user;
pub use sys_user::*;
mod sys_config;
pub use sys_config::*;
mod sys_notice;
pub use sys_notice::*;
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
