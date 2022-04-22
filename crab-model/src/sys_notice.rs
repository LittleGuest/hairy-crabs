use rbatis::crud_table;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// 通知公告表
#[crud_table]
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct SysNotice {
    /// id
    #[validate(length(max = 64))]
    pub id: Option<String>,
    /// 公告标题
    #[validate(length(max = 50))]
    pub notice_title: Option<String>,
    /// 公告类型（1通知 2公告）
    #[validate(length(max = 1))]
    pub notice_type: Option<String>,
    /// 公告内容
    pub notice_content: Option<Vec<u8>>,
    /// 公告内容HTML
    pub notice_content_html: Option<Vec<u8>>,
    /// 公告状态（0正常 1关闭）
    #[validate(length(max = 1))]
    pub status: Option<String>,
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
    /// 备注
    #[validate(length(max = 255))]
    pub remark: Option<String>,
    /// 版本
    pub version: Option<i32>,
    /// 删除标志（0代表存在 1代表删除）
    #[validate(length(max = 1))]
    pub del_flag: Option<String>,
    /// 更新IP
    #[validate(length(max = 128))]
    pub update_ip: Option<String>,
    /// 创建部门
    #[validate(length(max = 64))]
    pub create_dept: Option<String>,
}
