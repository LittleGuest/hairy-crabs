use serde::Serialize;

use crate::SysMenu;

#[derive(Serialize, PartialEq, Eq, Hash, Clone)]
pub struct SysMenuTreeDto {
    pub has_child: bool,
    pub children: Vec<SysMenuTreeDto>,

    /// ID
    pub id: Option<String>,
    /// 菜单编码
    pub menu_code: Option<String>,
    /// 菜单名称
    pub menu_name: Option<String>,
    /// 父菜单ID
    pub parent_id: Option<String>,
    /// 显示顺序
    pub sort: Option<i32>,
    /// 路由地址
    pub path: Option<String>,
    /// 组件路径
    pub component: Option<String>,
    /// 是否为外链（0是 1否）
    pub is_frame: Option<i32>,
    /// 是否缓存（0缓存 1不缓存）
    pub is_cache: Option<i32>,
    /// 菜单类型（M目录 C菜单 F按钮）
    pub menu_type: Option<String>,
    /// 菜单状态（0显示 1隐藏）
    pub visible: Option<String>,
    /// 菜单状态（0正常 1停用）
    pub status: Option<String>,
    /// 权限标识
    pub perms: Option<String>,
    /// 菜单图标
    pub icon: Option<String>,
    /// 创建者
    pub create_by: Option<String>,
    /// 创建时间
    pub create_time: Option<rbatis::DateTimeNative>,
    /// 更新者
    pub update_by: Option<String>,
    /// 更新时间
    pub update_time: Option<rbatis::DateTimeNative>,
    /// 备注
    pub remark: Option<String>,
    /// 父id集合
    pub parent_ids: Option<String>,
    /// 排序
    pub tree_sort: Option<i32>,
    /// 排序集合
    pub tree_sorts: Option<String>,
    /// 层级
    pub tree_level: Option<i32>,
    /// 是否子节点（0是 1否）
    pub tree_leaf: Option<String>,
    /// 创建部门
    pub create_dept: Option<String>,
    /// 更新IP
    pub update_ip: Option<String>,
    /// 版本
    pub version: Option<i32>,
    /// 删除标志（0代表存在 1代表删除）
    pub del_flag: Option<String>,
}

impl From<SysMenu> for SysMenuTreeDto {
    fn from(sm: SysMenu) -> Self {
        Self {
            has_child: false,
            children: vec![],
            id: sm.id,
            menu_code: sm.menu_code,
            menu_name: sm.menu_name,
            parent_id: sm.parent_id,
            sort: sm.sort,
            path: sm.path,
            component: sm.component,
            is_frame: sm.is_frame,
            is_cache: sm.is_cache,
            menu_type: sm.menu_type,
            visible: sm.visible,
            status: sm.status,
            perms: sm.perms,
            icon: sm.icon,
            create_by: sm.create_by,
            create_time: sm.create_time,
            update_by: sm.update_by,
            update_time: sm.update_time,
            remark: sm.remark,
            parent_ids: sm.parent_ids,
            tree_sort: sm.tree_sort,
            tree_sorts: sm.tree_sorts,
            tree_level: sm.tree_level,
            tree_leaf: sm.tree_leaf,
            create_dept: sm.create_dept,
            update_ip: sm.update_ip,
            version: sm.version,
            del_flag: sm.del_flag,
        }
    }
}

impl From<&SysMenu> for SysMenuTreeDto {
    fn from(sm: &SysMenu) -> Self {
        Self {
            has_child: false,
            children: vec![],
            id: sm.id.clone(),
            menu_code: sm.menu_code.clone(),
            menu_name: sm.menu_name.clone(),
            parent_id: sm.parent_id.clone(),
            sort: sm.sort.clone(),
            path: sm.path.clone(),
            component: sm.component.clone(),
            is_frame: sm.is_frame.clone(),
            is_cache: sm.is_cache.clone(),
            menu_type: sm.menu_type.clone(),
            visible: sm.visible.clone(),
            status: sm.status.clone(),
            perms: sm.perms.clone(),
            icon: sm.icon.clone(),
            create_by: sm.create_by.clone(),
            create_time: sm.create_time.clone(),
            update_by: sm.update_by.clone(),
            update_time: sm.update_time.clone(),
            remark: sm.remark.clone(),
            parent_ids: sm.parent_ids.clone(),
            tree_sort: sm.tree_sort.clone(),
            tree_sorts: sm.tree_sorts.clone(),
            tree_level: sm.tree_level.clone(),
            tree_leaf: sm.tree_leaf.clone(),
            create_dept: sm.create_dept.clone(),
            update_ip: sm.update_ip.clone(),
            version: sm.version.clone(),
            del_flag: sm.del_flag.clone(),
        }
    }
}
