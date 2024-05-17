/// mod.rs 文件模板
pub const MOD_TEMPLATE: &str = r#"
use crab_common::{error::CrabError, result::CrabResult};
use rbatis::{crud::CRUDTable, Page, PageRequest};

{% for table_name, _ in table_names %}
mod {{table_name}};
pub use {{table_name}}::*;
{% endfor %}

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
"#;

/// 模型模板
pub const MODEL_TEMPLATE: &str = r#"
use crab_common::{error::CrabError, result::CrabResult, PageDto};
use rbatis::{
    crud::{CRUDTable, CRUD},
    crud_table, Page, PageRequest,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{Mapper, RB};

/// {{table.comment}}
#[crud_table]
#[derive(
    Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Validate,
)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct {{ struct_name }} { {% if has_columns %}{% for column in columns %}
    /// {{column.comment}}
    {%if column.field_type == "String" -%}#[validate(length(max = {{column.max_length}}))]{%- endif%}
    pub {{column.name}}: Option<{{column.field_type}}>,{% endfor %}{% endif %}
}

impl std::fmt::Display for {{ struct_name }} {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", serde_json::json!(self))
    }
}

#[crab_lib::async_trait::async_trait]
impl Mapper for {{ struct_name }} {
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

impl {{ struct_name }} {
    pub async fn page(req: &{{ struct_name }}Req) -> CrabResult<Page<Self>> {
        let mut sql = String::new();
        sql.push_str(
            format!(
                " select {} from {} where 1 = 1 ",
                Self::table_columns(),
                Self::table_name()
            )
            .as_str(),
        );

        {% if has_columns %}{% for column in columns %}
        if let Some({{column.name}}) = &req.{{column.name}} {
            {%if column.field_type == "String"%}
                sql.push_str(&format!(" and {} like '%{}%' ",  "{{column.name}}", {{column.name}}));
            {%else%}
                sql.push_str(&format!(" and {} = {} ",  "{{column.name}}", {{column.name}}));
            {%endif%}
        }
        {% endfor %}{% endif %}

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
pub struct {{ struct_name }}Req { 
    /// 开始时间
    pub start_at: Option<u64>,
    /// 结束时间
    pub end_at: Option<u64>,
    /// 分页参数
    pub page: Option<PageDto>,

    {% if has_columns %}{% for column in columns %}
    /// {{column.comment}}
    pub {{column.name}}: Option<{{column.field_type}}>,{% endfor %}{% endif %}
}

impl {{ struct_name }}Req {
    pub fn new_page_req(&self) -> PageRequest {
        if let Some(page) = &self.page {
            PageRequest::new_option(&page.page_no, &page.page_size)
        } else {
            PageRequest::default()
        }
    }
}

"#;
