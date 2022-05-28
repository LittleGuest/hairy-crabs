use crab_common::{error::CrabError, result::CrabResult, PageDto};
use rbatis::{
    crud::{CRUDTable, CRUD},
    crud_table, Page, PageRequest,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{Mapper, RB};

/// 系统访问记录
#[crud_table]
#[derive(
    Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Validate,
)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct SysLoginLog {
    /// ID
    pub id: Option<i64>,
    /// 用户账号
    #[validate(length(max = 50))]
    pub account: Option<String>,
    /// 登录IP地址
    #[validate(length(max = 128))]
    pub ip: Option<String>,
    /// 登录地点
    #[validate(length(max = 255))]
    pub login_location: Option<String>,
    /// 浏览器类型
    #[validate(length(max = 50))]
    pub browser: Option<String>,
    /// 操作系统
    #[validate(length(max = 50))]
    pub os: Option<String>,
    /// 登录状态（0成功 1失败）
    pub status: Option<i8>,
    /// 提示消息
    #[validate(length(max = 255))]
    pub msg: Option<String>,
    /// 访问时间
    pub login_at: Option<rbatis::DateTimeNative>,
    /// 备注
    #[validate(length(max = 500))]
    pub remark: Option<String>,
    /// 删除标志（0代表存在 1代表删除）
    pub del_flag: Option<i8>,
}

impl std::fmt::Display for SysLoginLog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", serde_json::json!(self))
    }
}

#[crab_lib::async_trait::async_trait]
impl Mapper for SysLoginLog {
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

impl SysLoginLog {
    pub async fn page(req: &SysLoginLogReq) -> CrabResult<Page<Self>> {
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

        if let Some(account) = &req.account {
            sql.push_str(&format!(" and {} like '%{}%' ", "account", account));
        }

        if let Some(ip) = &req.ip {
            sql.push_str(&format!(" and {} like '%{}%' ", "ip", ip));
        }

        if let Some(login_location) = &req.login_location {
            sql.push_str(&format!(
                " and {} like '%{}%' ",
                "login_location", login_location
            ));
        }

        if let Some(browser) = &req.browser {
            sql.push_str(&format!(" and {} like '%{}%' ", "browser", browser));
        }

        if let Some(os) = &req.os {
            sql.push_str(&format!(" and {} like '%{}%' ", "os", os));
        }

        if let Some(status) = &req.status {
            sql.push_str(&format!(" and {} = {} ", "status", status));
        }

        if let Some(msg) = &req.msg {
            sql.push_str(&format!(" and {} like '%{}%' ", "msg", msg));
        }

        if let Some(login_at) = &req.login_at {
            sql.push_str(&format!(" and {} = {} ", "login_at", login_at));
        }

        if let Some(remark) = &req.remark {
            sql.push_str(&format!(" and {} like '%{}%' ", "remark", remark));
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
pub struct SysLoginLogReq {
    /// 开始时间
    pub start_at: Option<u64>,
    /// 结束时间
    pub end_at: Option<u64>,
    /// 分页参数
    pub page: Option<PageDto>,

    /// ID
    pub id: Option<i64>,
    /// 用户账号
    pub account: Option<String>,
    /// 登录IP地址
    pub ip: Option<String>,
    /// 登录地点
    pub login_location: Option<String>,
    /// 浏览器类型
    pub browser: Option<String>,
    /// 操作系统
    pub os: Option<String>,
    /// 登录状态（0成功 1失败）
    pub status: Option<i8>,
    /// 提示消息
    pub msg: Option<String>,
    /// 访问时间
    pub login_at: Option<rbatis::DateTimeNative>,
    /// 备注
    pub remark: Option<String>,
    /// 删除标志（0代表存在 1代表删除）
    pub del_flag: Option<i8>,
}

impl SysLoginLogReq {
    pub fn new_page_req(&self) -> PageRequest {
        if let Some(page) = &self.page {
            PageRequest::new_option(&page.page_no, &page.page_size)
        } else {
            PageRequest::default()
        }
    }
}
