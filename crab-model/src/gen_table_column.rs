use crab_common::{error::CrabError, result::CrabResult, PageDto};
use rbatis::{
    crud::{CRUDTable, CRUD},
    crud_table, Page, PageRequest,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{Mapper, RB};

/// 代码生成业务表字段
#[crud_table]
#[derive(
    Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Validate,
)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct GenTableColumn {
    /// 编号
    pub id: Option<i64>,
    /// 归属表编号
    #[validate(length(max = 64))]
    pub table_id: Option<String>,
    /// 列名称
    #[validate(length(max = 200))]
    pub column_name: Option<String>,
    /// 列描述
    #[validate(length(max = 500))]
    pub column_comment: Option<String>,
    /// 列类型
    #[validate(length(max = 100))]
    pub column_type: Option<String>,
    /// JAVA类型
    #[validate(length(max = 500))]
    pub java_type: Option<String>,
    /// JAVA字段名
    #[validate(length(max = 200))]
    pub java_field: Option<String>,
    /// 是否主键（1是）
    #[validate(length(max = 10))]
    pub is_pk: Option<String>,
    /// 是否自增（1是）
    #[validate(length(max = 10))]
    pub is_increment: Option<String>,
    /// 是否必填（1是）
    #[validate(length(max = 10))]
    pub is_required: Option<String>,
    /// 是否为插入字段（1是）
    #[validate(length(max = 10))]
    pub is_insert: Option<String>,
    /// 是否编辑字段（1是）
    #[validate(length(max = 10))]
    pub is_edit: Option<String>,
    /// 是否列表字段（1是）
    #[validate(length(max = 10))]
    pub is_list: Option<String>,
    /// 是否查询字段（1是）
    #[validate(length(max = 10))]
    pub is_query: Option<String>,
    /// 是否唯一性
    #[validate(length(max = 10))]
    pub is_unique: Option<String>,
    /// 是否记录日志
    #[validate(length(max = 10))]
    pub is_log: Option<String>,
    /// 是否排序
    #[validate(length(max = 10))]
    pub is_column_sort: Option<String>,
    /// 新行
    #[validate(length(max = 10))]
    pub is_new_row: Option<String>,
    /// 列数
    pub col_span: Option<i32>,
    /// 对齐方式
    #[validate(length(max = 10))]
    pub align_type: Option<String>,
    /// 查询方式（等于、不等于、大于、小于、范围）
    #[validate(length(max = 200))]
    pub query_type: Option<String>,
    /// 显示类型（文本框、文本域、下拉框、复选框、单选框、日期控件）
    #[validate(length(max = 200))]
    pub html_type: Option<String>,
    /// 字典类型
    #[validate(length(max = 200))]
    pub dict_type: Option<String>,
    /// 字段校验
    #[validate(length(max = 100))]
    pub col_check: Option<String>,
    /// 排序
    pub sort: Option<i32>,
    /// 创建者
    pub create_by: Option<i64>,
    /// 创建时间
    pub create_at: Option<rbatis::DateTimeNative>,
    /// 更新者
    pub update_by: Option<i64>,
    /// 更新时间
    pub update_at: Option<rbatis::DateTimeNative>,
}

impl std::fmt::Display for GenTableColumn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", serde_json::json!(self))
    }
}

#[crab_lib::async_trait::async_trait]
impl Mapper for GenTableColumn {
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

impl GenTableColumn {
    pub async fn page(req: &GenTableColumnReq) -> CrabResult<Page<Self>> {
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

        if let Some(table_id) = &req.table_id {
            sql.push_str(&format!(" and {} like '%{}%' ", "table_id", table_id));
        }

        if let Some(column_name) = &req.column_name {
            sql.push_str(&format!(" and {} like '%{}%' ", "column_name", column_name));
        }

        if let Some(column_comment) = &req.column_comment {
            sql.push_str(&format!(
                " and {} like '%{}%' ",
                "column_comment", column_comment
            ));
        }

        if let Some(column_type) = &req.column_type {
            sql.push_str(&format!(" and {} like '%{}%' ", "column_type", column_type));
        }

        if let Some(java_type) = &req.java_type {
            sql.push_str(&format!(" and {} like '%{}%' ", "java_type", java_type));
        }

        if let Some(java_field) = &req.java_field {
            sql.push_str(&format!(" and {} like '%{}%' ", "java_field", java_field));
        }

        if let Some(is_pk) = &req.is_pk {
            sql.push_str(&format!(" and {} like '%{}%' ", "is_pk", is_pk));
        }

        if let Some(is_increment) = &req.is_increment {
            sql.push_str(&format!(
                " and {} like '%{}%' ",
                "is_increment", is_increment
            ));
        }

        if let Some(is_required) = &req.is_required {
            sql.push_str(&format!(" and {} like '%{}%' ", "is_required", is_required));
        }

        if let Some(is_insert) = &req.is_insert {
            sql.push_str(&format!(" and {} like '%{}%' ", "is_insert", is_insert));
        }

        if let Some(is_edit) = &req.is_edit {
            sql.push_str(&format!(" and {} like '%{}%' ", "is_edit", is_edit));
        }

        if let Some(is_list) = &req.is_list {
            sql.push_str(&format!(" and {} like '%{}%' ", "is_list", is_list));
        }

        if let Some(is_query) = &req.is_query {
            sql.push_str(&format!(" and {} like '%{}%' ", "is_query", is_query));
        }

        if let Some(is_unique) = &req.is_unique {
            sql.push_str(&format!(" and {} like '%{}%' ", "is_unique", is_unique));
        }

        if let Some(is_log) = &req.is_log {
            sql.push_str(&format!(" and {} like '%{}%' ", "is_log", is_log));
        }

        if let Some(is_column_sort) = &req.is_column_sort {
            sql.push_str(&format!(
                " and {} like '%{}%' ",
                "is_column_sort", is_column_sort
            ));
        }

        if let Some(is_new_row) = &req.is_new_row {
            sql.push_str(&format!(" and {} like '%{}%' ", "is_new_row", is_new_row));
        }

        if let Some(col_span) = &req.col_span {
            sql.push_str(&format!(" and {} = {} ", "col_span", col_span));
        }

        if let Some(align_type) = &req.align_type {
            sql.push_str(&format!(" and {} like '%{}%' ", "align_type", align_type));
        }

        if let Some(query_type) = &req.query_type {
            sql.push_str(&format!(" and {} like '%{}%' ", "query_type", query_type));
        }

        if let Some(html_type) = &req.html_type {
            sql.push_str(&format!(" and {} like '%{}%' ", "html_type", html_type));
        }

        if let Some(dict_type) = &req.dict_type {
            sql.push_str(&format!(" and {} like '%{}%' ", "dict_type", dict_type));
        }

        if let Some(col_check) = &req.col_check {
            sql.push_str(&format!(" and {} like '%{}%' ", "col_check", col_check));
        }

        if let Some(sort) = &req.sort {
            sql.push_str(&format!(" and {} = {} ", "sort", sort));
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
pub struct GenTableColumnReq {
    /// 开始时间
    pub start_at: Option<u64>,
    /// 结束时间
    pub end_at: Option<u64>,
    /// 分页参数
    pub page: Option<PageDto>,

    /// 编号
    pub id: Option<i64>,
    /// 归属表编号
    pub table_id: Option<String>,
    /// 列名称
    pub column_name: Option<String>,
    /// 列描述
    pub column_comment: Option<String>,
    /// 列类型
    pub column_type: Option<String>,
    /// JAVA类型
    pub java_type: Option<String>,
    /// JAVA字段名
    pub java_field: Option<String>,
    /// 是否主键（1是）
    pub is_pk: Option<String>,
    /// 是否自增（1是）
    pub is_increment: Option<String>,
    /// 是否必填（1是）
    pub is_required: Option<String>,
    /// 是否为插入字段（1是）
    pub is_insert: Option<String>,
    /// 是否编辑字段（1是）
    pub is_edit: Option<String>,
    /// 是否列表字段（1是）
    pub is_list: Option<String>,
    /// 是否查询字段（1是）
    pub is_query: Option<String>,
    /// 是否唯一性
    pub is_unique: Option<String>,
    /// 是否记录日志
    pub is_log: Option<String>,
    /// 是否排序
    pub is_column_sort: Option<String>,
    /// 新行
    pub is_new_row: Option<String>,
    /// 列数
    pub col_span: Option<i32>,
    /// 对齐方式
    pub align_type: Option<String>,
    /// 查询方式（等于、不等于、大于、小于、范围）
    pub query_type: Option<String>,
    /// 显示类型（文本框、文本域、下拉框、复选框、单选框、日期控件）
    pub html_type: Option<String>,
    /// 字典类型
    pub dict_type: Option<String>,
    /// 字段校验
    pub col_check: Option<String>,
    /// 排序
    pub sort: Option<i32>,
    /// 创建者
    pub create_by: Option<i64>,
    /// 创建时间
    pub create_at: Option<rbatis::DateTimeNative>,
    /// 更新者
    pub update_by: Option<i64>,
    /// 更新时间
    pub update_at: Option<rbatis::DateTimeNative>,
}

impl GenTableColumnReq {
    pub fn new_page_req(&self) -> PageRequest {
        if let Some(page) = &self.page {
            PageRequest::new_option(&page.page_no, &page.page_size)
        } else {
            PageRequest::default()
        }
    }
}
