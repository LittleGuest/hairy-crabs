use rbatis::{crud_table, Timestamp};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// 代码生成业务表字段
#[crud_table]
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct GenTableColumn {
    /// 编号
    pub column_id: Option<i64>,
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
    #[validate(length(max = 64))]
    pub create_by: Option<String>,
    /// 创建时间
    pub create_time: Option<rbatis::DateTimeNative>,
    /// 更新者
    #[validate(length(max = 64))]
    pub update_by: Option<String>,
    /// 更新时间
    pub update_time: Option<rbatis::DateTimeNative>,
}
