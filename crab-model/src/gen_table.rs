use crab_lib::validator::Validate;
use rbatis::crud_table;
use serde::{Deserialize, Serialize};

/// 代码生成业务表
#[crud_table]
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct GenTable {
    /// 编号
    pub table_id: Option<i64>,
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
    #[validate(length(max = 1))]
    pub gen_type: Option<String>,
    ///
    #[validate(length(max = 4000))]
    pub options: Option<String>,
    /// 创建者
    #[validate(length(max = 64))]
    pub create_by: Option<String>,
    /// 创建时间
    pub create_time: Option<rbatis::DateTimeUtc>,
    /// 更新者
    #[validate(length(max = 64))]
    pub update_by: Option<String>,
    /// 更新时间
    pub update_time: Option<rbatis::DateTimeNative>,
    /// 备注
    #[validate(length(max = 500))]
    pub remark: Option<String>,
}
