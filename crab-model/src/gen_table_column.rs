use crab_common::{error::CrabError, result::CrabResult};
use rbatis::{crud::CRUD, crud_table};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{Mapper, RB};

/// 代码生成业务表字段
#[crud_table]
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Validate)]
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

impl GenTableColumn {}
