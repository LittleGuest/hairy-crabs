use crab_common::{error::CrabError, result::CrabResult, PageDto};
use rbatis::{
    crud::{CRUDTable, CRUD},
    crud_table, Page, PageRequest,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{Mapper, RB};

/// 字典数据表
#[crud_table]
#[derive(
    Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Validate,
)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct SysDictData {
    /// ID
    pub id: Option<i64>,
    /// 字典类型
    #[validate(length(max = 100))]
    pub dict_type: Option<String>,
    /// 字典编码
    pub code: Option<i64>,
    /// 字典标签
    #[validate(length(max = 100))]
    pub label: Option<String>,
    /// 字典键值
    #[validate(length(max = 100))]
    pub value: Option<String>,
    /// 是否默认（1是 0否）
    pub is_default: Option<i8>,
    /// 字典排序
    pub sort: Option<i16>,
    /// 样式属性（其他样式扩展）
    #[validate(length(max = 100))]
    pub css_class: Option<String>,
    /// 表格回显样式
    #[validate(length(max = 100))]
    pub table_class: Option<String>,
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

impl std::fmt::Display for SysDictData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", serde_json::json!(self))
    }
}

#[crab_lib::async_trait::async_trait]
impl Mapper for SysDictData {
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

impl SysDictData {
    pub async fn page(req: &SysDictDataReq) -> CrabResult<Page<Self>> {
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

        if let Some(dict_type) = &req.dict_type {
            sql.push_str(&format!(" and {} like '%{}%' ", "dict_type", dict_type));
        }

        if let Some(code) = &req.code {
            sql.push_str(&format!(" and {} = {} ", "code", code));
        }

        if let Some(label) = &req.label {
            sql.push_str(&format!(" and {} like '%{}%' ", "label", label));
        }

        if let Some(value) = &req.value {
            sql.push_str(&format!(" and {} like '%{}%' ", "value", value));
        }

        if let Some(is_default) = &req.is_default {
            sql.push_str(&format!(" and {} = {} ", "is_default", is_default));
        }

        if let Some(sort) = &req.sort {
            sql.push_str(&format!(" and {} = {} ", "sort", sort));
        }

        if let Some(css_class) = &req.css_class {
            sql.push_str(&format!(" and {} like '%{}%' ", "css_class", css_class));
        }

        if let Some(table_class) = &req.table_class {
            sql.push_str(&format!(" and {} like '%{}%' ", "table_class", table_class));
        }

        if let Some(status) = &req.status {
            sql.push_str(&format!(" and {} = {} ", "status", status));
        }

        if let Some(remark) = &req.remark {
            sql.push_str(&format!(" and {} like '%{}%' ", "remark", remark));
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
pub struct SysDictDataReq {
    /// 开始时间
    pub start_at: Option<u64>,
    /// 结束时间
    pub end_at: Option<u64>,
    /// 分页参数
    pub page: Option<PageDto>,

    /// ID
    pub id: Option<i64>,
    /// 字典类型
    pub dict_type: Option<String>,
    /// 字典编码
    pub code: Option<i64>,
    /// 字典标签
    pub label: Option<String>,
    /// 字典键值
    pub value: Option<String>,
    /// 是否默认（1是 0否）
    pub is_default: Option<i8>,
    /// 字典排序
    pub sort: Option<i16>,
    /// 样式属性（其他样式扩展）
    pub css_class: Option<String>,
    /// 表格回显样式
    pub table_class: Option<String>,
    /// 状态（0正常 1停用）
    pub status: Option<i8>,
    /// 备注
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

impl SysDictDataReq {
    pub fn new_page_req(&self) -> PageRequest {
        if let Some(page) = &self.page {
            PageRequest::new_option(&page.page_no, &page.page_size)
        } else {
            PageRequest::default()
        }
    }
}
