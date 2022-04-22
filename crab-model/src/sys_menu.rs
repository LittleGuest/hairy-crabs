use rbatis::crud_table;
use serde::{Deserialize, Serialize};
use validator::Validate;

///
#[crud_table]
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Hash, Deserialize, Validate)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct SysMenu {
    /// ID
    #[validate(length(max = 64))]
    pub id: Option<String>,
    /// 菜单编码
    #[validate(length(max = 500))]
    pub menu_code: Option<String>,
    /// 菜单名称
    #[validate(length(max = 50))]
    pub menu_name: Option<String>,
    /// 父菜单ID
    #[validate(length(max = 64))]
    pub parent_id: Option<String>,
    /// 显示顺序
    pub sort: Option<i32>,
    /// 路由地址
    #[validate(length(max = 200))]
    pub path: Option<String>,
    /// 组件路径
    #[validate(length(max = 255))]
    pub component: Option<String>,
    /// 是否为外链（0是 1否）
    pub is_frame: Option<i32>,
    /// 是否缓存（0缓存 1不缓存）
    pub is_cache: Option<i32>,
    /// 菜单类型（M目录 C菜单 F按钮）
    #[validate(length(max = 1))]
    pub menu_type: Option<String>,
    /// 菜单状态（0显示 1隐藏）
    #[validate(length(max = 1))]
    pub visible: Option<String>,
    /// 菜单状态（0正常 1停用）
    #[validate(length(max = 1))]
    pub status: Option<String>,
    /// 权限标识
    #[validate(length(max = 100))]
    pub perms: Option<String>,
    /// 菜单图标
    #[validate(length(max = 100))]
    pub icon: Option<String>,
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
    #[validate(length(max = 500))]
    pub remark: Option<String>,
    /// 父id集合
    #[validate(length(max = 500))]
    pub parent_ids: Option<String>,
    /// 排序
    pub tree_sort: Option<i32>,
    /// 排序集合
    #[validate(length(max = 500))]
    pub tree_sorts: Option<String>,
    /// 层级
    pub tree_level: Option<i32>,
    /// 是否子节点（0是 1否）
    #[validate(length(max = 1))]
    pub tree_leaf: Option<String>,
    /// 创建部门
    #[validate(length(max = 64))]
    pub create_dept: Option<String>,
    /// 更新IP
    #[validate(length(max = 128))]
    pub update_ip: Option<String>,
    /// 版本
    pub version: Option<i32>,
    /// 删除标志（0代表存在 1代表删除）
    #[validate(length(max = 1))]
    pub del_flag: Option<String>,
}
