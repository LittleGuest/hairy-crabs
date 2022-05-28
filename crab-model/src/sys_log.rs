use crab_common::{error::CrabError, result::CrabResult, PageDto};
use rbatis::{
    crud::{CRUDTable, CRUD},
    crud_table, Page, PageRequest,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{Mapper, RB};

/// 操作日志记录
#[crud_table]
#[derive(
    Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Validate,
)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct SysLog {
    /// ID
    pub id: Option<i64>,
    /// 模块标题
    #[validate(length(max = 50))]
    pub title: Option<String>,
    /// 业务类型（0其它 1新增 2修改 3删除）
    pub business_type: Option<i8>,
    /// 方法名称
    #[validate(length(max = 100))]
    pub method: Option<String>,
    /// 请求方式
    #[validate(length(max = 10))]
    pub request_method: Option<String>,
    /// 操作类别（0其它 1后台用户 2手机端用户）
    pub operator_type: Option<i8>,
    /// 操作人员
    #[validate(length(max = 50))]
    pub oper_name: Option<String>,
    /// 请求URL
    #[validate(length(max = 255))]
    pub oper_url: Option<String>,
    /// 主机地址
    #[validate(length(max = 128))]
    pub oper_ip: Option<String>,
    /// 操作地点
    #[validate(length(max = 255))]
    pub oper_location: Option<String>,
    /// 请求参数
    #[validate(length(max = 2000))]
    pub oper_param: Option<String>,
    /// 返回结果集
    #[validate(length(max = 65535))]
    pub json_result: Option<String>,
    /// 日志变更内容
    #[validate(length(max = 65535))]
    pub log_content: Option<String>,
    /// 操作状态（0正常 1异常）
    pub status: Option<i8>,
    /// 错误消息
    #[validate(length(max = 2000))]
    pub error_msg: Option<String>,
    /// 操作时间
    pub oper_time: Option<rbatis::DateTimeNative>,
    /// 备注
    #[validate(length(max = 500))]
    pub remark: Option<String>,
    /// 耗时
    pub take_up_time: Option<i32>,
    /// 创建者
    pub create_by: Option<i64>,
    /// 创建时间
    pub create_at: Option<rbatis::DateTimeNative>,
    /// 更新者
    pub update_by: Option<i64>,
    /// 更新时间
    pub update_at: Option<rbatis::DateTimeNative>,
    /// 删除标志（0代表存在 1代表删除）
    pub del_flag: Option<i8>,
}

impl std::fmt::Display for SysLog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", serde_json::json!(self))
    }
}

#[crab_lib::async_trait::async_trait]
impl Mapper for SysLog {
    async fn save(&self) -> CrabResult<Option<i64>> {
        let res = RB.save(self, &[]).await.map_err(|e| {
            log::error!("Mapper::save error {}", e);
            CrabError::SqlError
        })?;
        Ok(res.last_insert_id)
    }
    async fn save_batch(models: &[Self]) -> CrabResult<u64> {
        let res = RB.save_batch(models, &[]).await.map_err(|e| {
            log::error!("Mapper::save_batch error {}", e);
            CrabError::SqlError
        })?;
        Ok(res.rows_affected)
    }
    async fn update(&self) -> CrabResult<u64> {
        let w = RB.new_wrapper().eq("id", self.id);
        let res = RB.update_by_wrapper(self, w, &[]).await.map_err(|e| {
            log::error!("Mapper::update error {}", e);
            CrabError::SqlError
        })?;
        Ok(res)
    }
    async fn update_batch(models: &[Self]) -> CrabResult<u64> {
        let mut res = 0;
        for m in models.iter() {
            res += m.update().await?;
        }
        Ok(res)
    }
    async fn remove_by_id(id: i64) -> CrabResult<u64> {
        let res = RB
            .remove_by_column::<Self, _>("id", id)
            .await
            .map_err(|e| {
                log::error!("Mapper::remove_by_id error {}", e);
                CrabError::SqlError
            })?;
        Ok(res)
    }
    async fn remove_batch_by_ids(ids: &[i64]) -> CrabResult<u64> {
        let res = RB
            .remove_batch_by_column::<Self, _>("id", ids)
            .await
            .map_err(|e| {
                log::error!("Mapper::remove_batch_by_ids error {}", e);
                CrabError::SqlError
            })?;
        Ok(res)
    }
    async fn list() -> CrabResult<Vec<Self>> {
        let res = RB.fetch_list().await.map_err(|e| {
            log::error!("Mapper::list error {}", e);
            CrabError::SqlError
        })?;
        Ok(res)
    }
    async fn fetch_by_id(id: i64) -> CrabResult<Option<Self>> {
        let res = RB.fetch_by_column("id", id).await.map_err(|e| {
            log::error!("Mapper::fetch_by_id error {}", e);
            CrabError::SqlError
        })?;
        Ok(res)
    }
    async fn fetch_by_ids(ids: &[i64]) -> CrabResult<Vec<Self>> {
        let res = RB.fetch_list_by_column("id", ids).await.map_err(|e| {
            log::error!("Mapper::fetch_by_ids error {}", e);
            CrabError::SqlError
        })?;
        Ok(res)
    }
}

impl SysLog {
    pub async fn page(req: &SysLogReq) -> CrabResult<Page<Self>> {
        let mut sql = String::new();
        sql.push_str(
            format!(
                " select {} from {} where 1 = 1 ",
                Self::table_columns(),
                Self::table_name()
            )
            .as_str(),
        );

        if let Some(id) = &req.id {
            sql.push_str(&format!(" and {} = {} ", "id", id));
        }

        if let Some(title) = &req.title {
            sql.push_str(&format!(" and {} like '%{}%' ", "title", title));
        }

        if let Some(business_type) = &req.business_type {
            sql.push_str(&format!(" and {} = {} ", "business_type", business_type));
        }

        if let Some(method) = &req.method {
            sql.push_str(&format!(" and {} like '%{}%' ", "method", method));
        }

        if let Some(request_method) = &req.request_method {
            sql.push_str(&format!(
                " and {} like '%{}%' ",
                "request_method", request_method
            ));
        }

        if let Some(operator_type) = &req.operator_type {
            sql.push_str(&format!(" and {} = {} ", "operator_type", operator_type));
        }

        if let Some(oper_name) = &req.oper_name {
            sql.push_str(&format!(" and {} like '%{}%' ", "oper_name", oper_name));
        }

        if let Some(oper_url) = &req.oper_url {
            sql.push_str(&format!(" and {} like '%{}%' ", "oper_url", oper_url));
        }

        if let Some(oper_ip) = &req.oper_ip {
            sql.push_str(&format!(" and {} like '%{}%' ", "oper_ip", oper_ip));
        }

        if let Some(oper_location) = &req.oper_location {
            sql.push_str(&format!(
                " and {} like '%{}%' ",
                "oper_location", oper_location
            ));
        }

        if let Some(oper_param) = &req.oper_param {
            sql.push_str(&format!(" and {} like '%{}%' ", "oper_param", oper_param));
        }

        if let Some(json_result) = &req.json_result {
            sql.push_str(&format!(" and {} like '%{}%' ", "json_result", json_result));
        }

        if let Some(log_content) = &req.log_content {
            sql.push_str(&format!(" and {} like '%{}%' ", "log_content", log_content));
        }

        if let Some(status) = &req.status {
            sql.push_str(&format!(" and {} = {} ", "status", status));
        }

        if let Some(error_msg) = &req.error_msg {
            sql.push_str(&format!(" and {} like '%{}%' ", "error_msg", error_msg));
        }

        if let Some(oper_time) = &req.oper_time {
            sql.push_str(&format!(" and {} = {} ", "oper_time", oper_time));
        }

        if let Some(remark) = &req.remark {
            sql.push_str(&format!(" and {} like '%{}%' ", "remark", remark));
        }

        if let Some(take_up_time) = &req.take_up_time {
            sql.push_str(&format!(" and {} = {} ", "take_up_time", take_up_time));
        }

        if let Some(create_by) = &req.create_by {
            sql.push_str(&format!(" and {} = {} ", "create_by", create_by));
        }

        if let Some(create_at) = &req.create_at {
            sql.push_str(&format!(" and {} = {} ", "create_at", create_at));
        }

        if let Some(update_by) = &req.update_by {
            sql.push_str(&format!(" and {} = {} ", "update_by", update_by));
        }

        if let Some(update_at) = &req.update_at {
            sql.push_str(&format!(" and {} = {} ", "update_at", update_at));
        }

        if let Some(del_flag) = &req.del_flag {
            sql.push_str(&format!(" and {} = {} ", "del_flag", del_flag));
        }

        let res = RB
            .fetch_page(&sql, vec![], &req.new_page_req())
            .await
            .map_err(|e| {
                log::error!("page error {}", e);
                CrabError::SqlError
            })?;
        Ok(res)
    }
}

#[derive(Serialize, Deserialize)]
pub struct SysLogReq {
    /// 开始时间
    pub start_at: Option<u64>,
    /// 结束时间
    pub end_at: Option<u64>,
    /// 分页参数
    pub page: Option<PageDto>,

    /// ID
    pub id: Option<i64>,
    /// 模块标题
    pub title: Option<String>,
    /// 业务类型（0其它 1新增 2修改 3删除）
    pub business_type: Option<i8>,
    /// 方法名称
    pub method: Option<String>,
    /// 请求方式
    pub request_method: Option<String>,
    /// 操作类别（0其它 1后台用户 2手机端用户）
    pub operator_type: Option<i8>,
    /// 操作人员
    pub oper_name: Option<String>,
    /// 请求URL
    pub oper_url: Option<String>,
    /// 主机地址
    pub oper_ip: Option<String>,
    /// 操作地点
    pub oper_location: Option<String>,
    /// 请求参数
    pub oper_param: Option<String>,
    /// 返回结果集
    pub json_result: Option<String>,
    /// 日志变更内容
    pub log_content: Option<String>,
    /// 操作状态（0正常 1异常）
    pub status: Option<i8>,
    /// 错误消息
    pub error_msg: Option<String>,
    /// 操作时间
    pub oper_time: Option<rbatis::DateTimeNative>,
    /// 备注
    pub remark: Option<String>,
    /// 耗时
    pub take_up_time: Option<i32>,
    /// 创建者
    pub create_by: Option<i64>,
    /// 创建时间
    pub create_at: Option<rbatis::DateTimeNative>,
    /// 更新者
    pub update_by: Option<i64>,
    /// 更新时间
    pub update_at: Option<rbatis::DateTimeNative>,
    /// 删除标志（0代表存在 1代表删除）
    pub del_flag: Option<i8>,
}

impl SysLogReq {
    pub fn new_page_req(&self) -> PageRequest {
        if let Some(page) = &self.page {
            PageRequest::new_option(&page.page_no, &page.page_size)
        } else {
            PageRequest::default()
        }
    }
}
