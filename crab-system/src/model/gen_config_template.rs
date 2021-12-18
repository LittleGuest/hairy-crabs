use rbatis::{crud_table, Timestamp};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// 模板配置表
#[crud_table]
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct GenConfigTemplate {
    /// 主键ID
    #[validate(length(max = 50))]
    pub id: Option<String>,
    /// 模板名称
    #[validate(length(max = 50))]
    pub template_name: Option<String>,
    /// 作者
    #[validate(length(max = 100))]
    pub function_author: Option<String>,
    /// 邮箱
    #[validate(length(max = 100))]
    pub function_author_email: Option<String>,
    /// 工作空间路径
    #[validate(length(max = 200))]
    pub workspace_path: Option<String>,
    /// 模块名
    #[validate(length(max = 30))]
    pub module_name: Option<String>,
    /// 模块包路径
    #[validate(length(max = 100))]
    pub package_name: Option<String>,
    /// 前端工作空间路径
    #[validate(length(max = 200))]
    pub web_workspace_path: Option<String>,
    /// 是否默认
    #[validate(length(max = 10))]
    pub template_default: Option<String>,
    /// 排序
    pub sort: Option<i32>,
    /// 状态（0正常 1 停用）
    #[validate(length(max = 1))]
    pub status: Option<String>,
    /// 创建者
    #[validate(length(max = 64))]
    pub create_by: Option<String>,
    /// 创建部门
    #[validate(length(max = 64))]
    pub create_dept: Option<String>,
    /// 创建时间
    pub create_time: Option<rbatis::DateTimeNative>,
    /// 更新者
    #[validate(length(max = 64))]
    pub update_by: Option<String>,
    /// 更新时间
    pub update_time: Option<rbatis::DateTimeNative>,
    /// 更新IP
    #[validate(length(max = 128))]
    pub update_ip: Option<String>,
    /// 备注
    #[validate(length(max = 500))]
    pub remark: Option<String>,
    /// 版本
    pub version: Option<i32>,
    /// 删除标志（0代表存在 1代表删除）
    #[validate(length(max = 1))]
    pub del_flag: Option<String>,
}
