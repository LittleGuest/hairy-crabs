use crab_common::{error::CrabError, result::CrabResult, PageDto};
use rbatis::{
    crud::{CRUDTable, CRUD},
    crud_table, Page, PageRequest,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{Mapper, RB};

/// 字典类型表
#[crud_table]
#[derive(
    Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Validate,
)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct SysDictType {
    /// ID
    pub id: Option<i64>,
    /// 字典名称
    #[validate(length(max = 100))]
    pub name: Option<String>,
    /// 字典类型
    #[validate(length(max = 100))]
    pub r#type: Option<String>,
    /// 状态（0正常 1停用）
    pub status: Option<i8>,
    /// 备注
    #[validate(length(max = 500))]
    pub remark: Option<String>,
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

impl std::fmt::Display for SysDictType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", serde_json::json!(self))
    }
}

#[crab_lib::async_trait::async_trait]
impl Mapper for SysDictType {
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

impl SysDictType {
    pub async fn page(req: &SysDictTypeReq) -> CrabResult<Page<Self>> {
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

        if let Some(name) = &req.name {
            sql.push_str(&format!(" and {} like '%{}%' ", "name", name));
        }

        if let Some(r#type) = &req.r#type {
            sql.push_str(&format!(" and {} like '%{}%' ", "r#type", r#type));
        }

        if let Some(status) = &req.status {
            sql.push_str(&format!(" and {} = {} ", "status", status));
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
pub struct SysDictTypeReq {
    /// 开始时间
    pub start_at: Option<u64>,
    /// 结束时间
    pub end_at: Option<u64>,
    /// 分页参数
    pub page: Option<PageDto>,

    /// ID
    pub id: Option<i64>,
    /// 字典名称
    pub name: Option<String>,
    /// 字典类型
    pub r#type: Option<String>,
    /// 状态（0正常 1停用）
    pub status: Option<i8>,
}

impl SysDictTypeReq {
    pub fn new_page_req(&self) -> PageRequest {
        if let Some(page) = &self.page {
            PageRequest::new_option(&page.page_no, &page.page_size)
        } else {
            PageRequest::default()
        }
    }
}
