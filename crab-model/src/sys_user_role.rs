use crab_common::{error::CrabError, result::CrabResult, PageDto};
use rbatis::{
    crud::{CRUDTable, CRUD},
    crud_table, Page, PageRequest,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{Mapper, RB};

/// 用户和角色关联表
#[crud_table]
#[derive(
    Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Validate,
)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct SysUserRole {
    /// 用户ID
    pub user_id: Option<i64>,
    /// 角色ID
    pub role_id: Option<i64>,
}

impl std::fmt::Display for SysUserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", serde_json::json!(self))
    }
}

#[crab_lib::async_trait::async_trait]
impl Mapper for SysUserRole {
    async fn save(&self) -> CrabResult<Option<i64>> {
        unimplemented!()
    }
    async fn save_batch(models: &[Self]) -> CrabResult<u64> {
        let res = RB.save_batch(models, &[]).await.map_err(|e| {
            log::error!("Mapper::save_batch error {}", e);
            CrabError::SqlError
        })?;
        Ok(res.rows_affected)
    }
    async fn update(&self) -> CrabResult<u64> {
        unimplemented!()
    }
    async fn update_batch(_models: &[Self]) -> CrabResult<u64> {
        unimplemented!()
    }
    async fn remove_by_id(_id: i64) -> CrabResult<u64> {
        unimplemented!()
    }
    async fn remove_batch_by_ids(_ids: &[i64]) -> CrabResult<u64> {
        unimplemented!()
    }
    async fn list() -> CrabResult<Vec<Self>> {
        let res = RB.fetch_list().await.map_err(|e| {
            log::error!("Mapper::list error {}", e);
            CrabError::SqlError
        })?;
        Ok(res)
    }
    async fn fetch_by_id(_id: i64) -> CrabResult<Option<Self>> {
        unimplemented!()
    }
    async fn fetch_by_ids(_ids: &[i64]) -> CrabResult<Vec<Self>> {
        unimplemented!()
    }
}

impl SysUserRole {
    pub async fn page(req: &SysUserRoleReq) -> CrabResult<Page<Self>> {
        let mut sql = String::new();
        sql.push_str(
            format!(
                " select {} from {} where 1 = 1 ",
                Self::table_columns(),
                Self::table_name()
            )
            .as_str(),
        );

        if let Some(user_id) = &req.user_id {
            sql.push_str(&format!(" and {} = {} ", "user_id", user_id));
        }

        if let Some(role_id) = &req.role_id {
            sql.push_str(&format!(" and {} = {} ", "role_id", role_id));
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

    pub async fn fetch_by_user_id(uid: i64) -> CrabResult<Vec<Self>> {
        Self::fetch_by_column("user_id", uid).await
    }
    pub async fn fetch_by_role_id(rid: i64) -> CrabResult<Vec<Self>> {
        Self::fetch_by_column("role_id", rid).await
    }

    async fn fetch_by_column(col: &str, v: i64) -> CrabResult<Vec<Self>> {
        let res = RB.fetch_list_by_column(col, &[v]).await.map_err(|e| {
            log::error!("Mapper::fetch_by_column error {}", e);
            CrabError::SqlError
        })?;
        Ok(res)
    }

    pub async fn remove_by_user_id(uid: i64) -> CrabResult<u64> {
        Self::remove_by_column("user_id", uid).await
    }

    pub async fn remove_by_role_id(rid: i64) -> CrabResult<u64> {
        Self::remove_by_column("role_id", rid).await
    }

    async fn remove_by_column(col: &str, v: i64) -> CrabResult<u64> {
        let res = RB.remove_by_column::<Self, _>(col, v).await.map_err(|e| {
            log::error!("Mapper::remove_by_column error {}", e);
            CrabError::SqlError
        })?;
        Ok(res)
    }

    pub async fn remove_by_user_role(&self) -> CrabResult<u64> {
        let w = RB
            .new_wrapper()
            .eq("user_id", self.user_id)
            .eq("role_id", self.role_id);
        let res = RB.remove_by_wrapper::<Self>(w).await.map_err(|e| {
            log::error!("Mapper::remove_by_column error {}", e);
            CrabError::SqlError
        })?;
        Ok(res)
    }
}

#[derive(Serialize, Deserialize)]
pub struct SysUserRoleReq {
    /// 分页参数
    pub page: Option<PageDto>,

    /// 用户ID
    pub user_id: Option<i64>,
    /// 角色ID
    pub role_id: Option<i64>,
}

impl SysUserRoleReq {
    pub fn new_page_req(&self) -> PageRequest {
        if let Some(page) = &self.page {
            PageRequest::new_option(&page.page_no, &page.page_size)
        } else {
            PageRequest::default()
        }
    }
}
