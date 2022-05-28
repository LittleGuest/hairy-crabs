use crab_common::{error::CrabError, result::CrabResult, PageDto};
use rbatis::{
    crud::{CRUDTable, CRUD},
    crud_table, Page, PageRequest,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{Mapper, RB};

/// 代码生成业务表
#[crud_table]
#[derive(
    Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Validate,
)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct GenTable {
    /// 编号
    pub id: Option<i64>,
    /// 表名称
    #[validate(length(max = 200))]
    pub table_name: Option<String>,
    /// 表描述
    #[validate(length(max = 500))]
    pub table_comment: Option<String>,
    /// 关联子表的表名
    #[validate(length(max = 64))]
    pub sub_table_name: Option<String>,
    /// 子表关联的外键名
    #[validate(length(max = 64))]
    pub sub_table_fk_name: Option<String>,
    /// 实体类名称
    #[validate(length(max = 100))]
    pub class_name: Option<String>,
    /// 使用的模板（crud单表操作 tree树表操作）
    #[validate(length(max = 200))]
    pub tpl_category: Option<String>,
    /// 工作空间
    #[validate(length(max = 200))]
    pub workspace_path: Option<String>,
    /// 模块名
    #[validate(length(max = 30))]
    pub module_name: Option<String>,
    /// 包路径
    #[validate(length(max = 100))]
    pub package_name: Option<String>,
    /// 业务名
    #[validate(length(max = 30))]
    pub business_name: Option<String>,
    /// 功能名
    #[validate(length(max = 50))]
    pub function_name: Option<String>,
    /// 作者
    #[validate(length(max = 50))]
    pub function_author: Option<String>,
    /// 邮箱
    #[validate(length(max = 200))]
    pub function_author_email: Option<String>,
    /// 前端工作空间路径
    #[validate(length(max = 200))]
    pub web_workspace_path: Option<String>,
    /// 生成代码方式（0zip压缩包 1自定义路径）
    pub gen_type: Option<i8>,
    /// 扩展选项
    #[validate(length(max = 4000))]
    pub options: Option<String>,
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
}

impl std::fmt::Display for GenTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", serde_json::json!(self))
    }
}

#[crab_lib::async_trait::async_trait]
impl Mapper for GenTable {
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

impl GenTable {
    pub async fn page(req: &GenTableReq) -> CrabResult<Page<Self>> {
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

        if let Some(table_name) = &req.table_name {
            sql.push_str(&format!(" and {} like '%{}%' ", "table_name", table_name));
        }

        if let Some(table_comment) = &req.table_comment {
            sql.push_str(&format!(
                " and {} like '%{}%' ",
                "table_comment", table_comment
            ));
        }

        if let Some(sub_table_name) = &req.sub_table_name {
            sql.push_str(&format!(
                " and {} like '%{}%' ",
                "sub_table_name", sub_table_name
            ));
        }

        if let Some(sub_table_fk_name) = &req.sub_table_fk_name {
            sql.push_str(&format!(
                " and {} like '%{}%' ",
                "sub_table_fk_name", sub_table_fk_name
            ));
        }

        if let Some(class_name) = &req.class_name {
            sql.push_str(&format!(" and {} like '%{}%' ", "class_name", class_name));
        }

        if let Some(tpl_category) = &req.tpl_category {
            sql.push_str(&format!(
                " and {} like '%{}%' ",
                "tpl_category", tpl_category
            ));
        }

        if let Some(workspace_path) = &req.workspace_path {
            sql.push_str(&format!(
                " and {} like '%{}%' ",
                "workspace_path", workspace_path
            ));
        }

        if let Some(module_name) = &req.module_name {
            sql.push_str(&format!(" and {} like '%{}%' ", "module_name", module_name));
        }

        if let Some(package_name) = &req.package_name {
            sql.push_str(&format!(
                " and {} like '%{}%' ",
                "package_name", package_name
            ));
        }

        if let Some(business_name) = &req.business_name {
            sql.push_str(&format!(
                " and {} like '%{}%' ",
                "business_name", business_name
            ));
        }

        if let Some(function_name) = &req.function_name {
            sql.push_str(&format!(
                " and {} like '%{}%' ",
                "function_name", function_name
            ));
        }

        if let Some(function_author) = &req.function_author {
            sql.push_str(&format!(
                " and {} like '%{}%' ",
                "function_author", function_author
            ));
        }

        if let Some(function_author_email) = &req.function_author_email {
            sql.push_str(&format!(
                " and {} like '%{}%' ",
                "function_author_email", function_author_email
            ));
        }

        if let Some(web_workspace_path) = &req.web_workspace_path {
            sql.push_str(&format!(
                " and {} like '%{}%' ",
                "web_workspace_path", web_workspace_path
            ));
        }

        if let Some(gen_type) = &req.gen_type {
            sql.push_str(&format!(" and {} = {} ", "gen_type", gen_type));
        }

        if let Some(options) = &req.options {
            sql.push_str(&format!(" and {} like '%{}%' ", "options", options));
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
pub struct GenTableReq {
    /// 开始时间
    pub start_at: Option<u64>,
    /// 结束时间
    pub end_at: Option<u64>,
    /// 分页参数
    pub page: Option<PageDto>,

    /// 编号
    pub id: Option<i64>,
    /// 表名称
    pub table_name: Option<String>,
    /// 表描述
    pub table_comment: Option<String>,
    /// 关联子表的表名
    pub sub_table_name: Option<String>,
    /// 子表关联的外键名
    pub sub_table_fk_name: Option<String>,
    /// 实体类名称
    pub class_name: Option<String>,
    /// 使用的模板（crud单表操作 tree树表操作）
    pub tpl_category: Option<String>,
    /// 工作空间
    pub workspace_path: Option<String>,
    /// 模块名
    pub module_name: Option<String>,
    /// 包路径
    pub package_name: Option<String>,
    /// 业务名
    pub business_name: Option<String>,
    /// 功能名
    pub function_name: Option<String>,
    /// 作者
    pub function_author: Option<String>,
    /// 邮箱
    pub function_author_email: Option<String>,
    /// 前端工作空间路径
    pub web_workspace_path: Option<String>,
    /// 生成代码方式（0zip压缩包 1自定义路径）
    pub gen_type: Option<i8>,
    /// 扩展选项
    pub options: Option<String>,
}

impl GenTableReq {
    pub fn new_page_req(&self) -> PageRequest {
        if let Some(page) = &self.page {
            PageRequest::new_option(&page.page_no, &page.page_size)
        } else {
            PageRequest::default()
        }
    }
}
