#![forbid(unsafe_code)]

use crab_common::{error::CrabError, result::CrabResult};
use crab_config::APP;
use crab_lib::lazy_static::lazy_static;
use rbatis::{
    crud::CRUDTable, db::DBPoolOptions, intercept::SqlIntercept,
    logic_delete::RbatisLogicDeletePlugin, rbatis::Rbatis,
};

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
mod sys_config;
pub use sys_config::*;
mod sys_login_log;
pub use sys_login_log::*;
mod sys_dict_type;
pub use sys_dict_type::*;
mod sys_dict_data;
pub use sys_dict_data::*;

lazy_static! {
    pub static ref RB: Rbatis = {
        let mut rb = Rbatis::new();
        // SQL拦截器
        rb.add_sql_intercept(SqlInterceptor);
        // 逻辑删除插件
        rb.set_logic_plugin(RbatisLogicDeletePlugin::new("del_flag"));
        rb
    };
}

/// 初始化数据库
pub async fn init_db() -> Result<(), CrabError> {
    log::info!("初始化数据库连接");
    let pool_options = DBPoolOptions::default();
    RB.link_opt(APP.database_url.as_str(), pool_options)
        .await
        .map_err(|e| {
            log::error!("数据库连接失败: {}", e);
            CrabError::ServerError("数据库连接失败")
        })?;
    log::info!("初始化数据库连接完成");
    Ok(())
}

#[crab_lib::async_trait::async_trait]
pub trait Mapper: CRUDTable + Sized {
    async fn save(&self) -> CrabResult<Option<i64>>;
    async fn save_batch(models: &[Self]) -> CrabResult<u64>;
    async fn update(&self) -> CrabResult<u64>;
    async fn update_batch(models: &[Self]) -> CrabResult<u64>;
    async fn remove_by_id(id: i64) -> CrabResult<u64>;
    async fn remove_batch_by_ids(ids: &[i64]) -> CrabResult<u64>;
    async fn list() -> CrabResult<Vec<Self>>;
    async fn fetch_by_id(id: i64) -> CrabResult<Option<Self>>;
    async fn fetch_by_ids(ids: &[i64]) -> CrabResult<Vec<Self>>;
    // async fn page(pr: PageRequest) -> Page<Self>;
}

/// SQL 拦截器
#[derive(Debug)]
struct SqlInterceptor;

impl SqlIntercept for SqlInterceptor {
    fn do_intercept(
        &self,
        _rb: &Rbatis,
        sql: &mut String,
        args: &mut Vec<rbson::Bson>,
        is_prepared_sql: bool,
    ) -> Result<(), rbatis::core::Error> {
        println!("SQL 拦截器");
        println!("{sql}, {:?}, {is_prepared_sql}", args);
        Ok(())
    }
}
