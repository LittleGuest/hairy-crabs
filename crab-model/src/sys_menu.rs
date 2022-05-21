use std::collections::HashSet;

use crab_common::{error::CrabError, result::CrabResult};
use rbatis::{crud::CRUD, crud_table};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{Mapper, RB};

/// 菜单信息表
#[crud_table]
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Validate, Hash)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct SysMenu {
    /// 主键ID
    pub id: Option<i64>,
    /// 菜单编码
    #[validate(length(max = 500))]
    pub menu_code: Option<String>,
    /// 菜单名称
    #[validate(length(max = 50))]
    pub menu_name: Option<String>,
    /// 父菜单ID
    pub pid: Option<i64>,
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
    pub visible: Option<i8>,
    /// 菜单状态（0正常 1停用）
    pub status: Option<i8>,
    /// 权限标识
    #[validate(length(max = 100))]
    pub perms: Option<String>,
    /// 菜单图标
    #[validate(length(max = 100))]
    pub icon: Option<String>,
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
    pub tree_leaf: Option<i8>,
    /// 创建者
    pub create_by: Option<i64>,
    /// 创建时间
    pub create_at: Option<rbatis::DateTimeNative>,
    /// 更新者
    pub update_by: Option<i64>,
    /// 更新时间
    pub update_at: Option<rbatis::DateTimeNative>,
    /// 删除标志（0代表存在 1代表删除）
    pub del_flag: Option<i8>,
}

impl std::fmt::Display for SysMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", serde_json::json!(self))
    }
}

#[crab_lib::async_trait::async_trait]
impl Mapper for SysMenu {
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

impl SysMenu {
    /// 根据用户ID查询菜单树信息
    pub async fn get_menu_by_user_id(_user_id: i64) -> Result<HashSet<Self>, CrabError> {
        let sql = "
        select
            distinct
                    t.id ,
            t.menu_name ,
            t.menu_code ,
            t.path ,
            t.component ,
            t.is_frame ,
            t.is_cache ,
            t.menu_type ,
            t.visible ,
            t.status ,
            ifnull(t.perms, '') ,
            t.icon ,
            t.pid ,
            t.parent_ids ,
            t.tree_sort ,
            t.tree_sorts ,
            t.tree_level ,
            t.tree_leaf ,
            t.create_by ,
            t.create_at ,
            t.update_by ,
            t.update_at ,
            t.remark ,
            t.del_flag
        from
            sys_menu t
        left join sys_role_menu rm on
            t.id = rm.menu_id
        left join sys_user_role ur on
            rm.role_id = ur.role_id
        left join sys_role ro on
            ur.role_id = ro.id
        left join sys_user u on
            ur.user_id = u.id
        where
            u.id = #{user_id}
            and t.del_flag = '0'
            and t.menu_type in ('M', 'C')
            and t.status = 0
            AND ro.status = 0
        order by
            t.tree_sorts
        ";
        let menus: Result<HashSet<Self>, CrabError> = RB
            .fetch(sql, vec![])
            .await
            .map_err(|_e| CrabError::ServerError("根据用户ID查询菜单树信息"));
        menus
    }

    /// 根据用户ID查询菜单树信息
    pub async fn menus() -> Result<HashSet<Self>, CrabError> {
        let sql = "
        select
            distinct
                    t.id ,
            t.menu_name ,
            t.menu_code ,
            t.path ,
            t.component ,
            t.is_frame ,
            t.is_cache ,
            t.menu_type ,
            t.visible ,
            t.status ,
            ifnull(t.perms, '') ,
            t.icon ,
            t.pid ,
            t.parent_ids ,
            t.tree_sort ,
            t.tree_sorts ,
            t.tree_level ,
            t.tree_leaf ,
            t.create_by ,
            t.create_at ,
            t.update_by ,
            t.update_at ,
            t.remark ,
            t.del_flag
        from
            sys_menu t
        where
            t.menu_type in ('M', 'C')
            and t.status = 0
            and t.del_flag = '0'
        order by
            t.tree_sorts
        ";
        let menus: Result<HashSet<Self>, CrabError> = RB
            .fetch(sql, vec![])
            .await
            .map_err(|_e| CrabError::ServerError("根据用户ID查询菜单树信息"));
        menus
    }
}

#[derive(Serialize, PartialEq, Eq, Hash, Clone)]
pub struct SysMenuTreeDto {
    pub has_child: bool,
    pub children: Vec<SysMenuTreeDto>,

    /// 主键ID
    pub id: Option<i64>,
    /// 菜单编码
    pub menu_code: Option<String>,
    /// 菜单名称
    pub menu_name: Option<String>,
    /// 父菜单ID
    pub pid: Option<i64>,
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
    pub visible: Option<i8>,
    /// 菜单状态（0正常 1停用）
    pub status: Option<i8>,
    /// 权限标识
    pub perms: Option<String>,
    /// 菜单图标
    pub icon: Option<String>,
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
    pub tree_leaf: Option<i8>,
    /// 创建者
    pub create_by: Option<i64>,
    /// 创建时间
    pub create_at: Option<rbatis::DateTimeNative>,
    /// 更新者
    pub update_by: Option<i64>,
    /// 更新时间
    pub update_at: Option<rbatis::DateTimeNative>,
    /// 删除标志（0代表存在 1代表删除）
    pub del_flag: Option<i8>,
}

impl From<SysMenu> for SysMenuTreeDto {
    fn from(sm: SysMenu) -> Self {
        Self {
            has_child: false,
            children: vec![],
            id: sm.id,
            menu_code: sm.menu_code,
            menu_name: sm.menu_name,
            pid: sm.pid,
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
            remark: sm.remark,
            parent_ids: sm.parent_ids,
            tree_sort: sm.tree_sort,
            tree_sorts: sm.tree_sorts,
            tree_level: sm.tree_level,
            tree_leaf: sm.tree_leaf,
            create_by: sm.create_by,
            create_at: sm.create_at,
            update_by: sm.update_by,
            update_at: sm.update_at,
            del_flag: sm.del_flag,
        }
    }
}

impl From<&SysMenu> for SysMenuTreeDto {
    fn from(sm: &SysMenu) -> Self {
        Self {
            has_child: false,
            children: vec![],
            id: sm.id,
            menu_code: sm.menu_code.clone(),
            menu_name: sm.menu_name.clone(),
            pid: sm.pid,
            sort: sm.sort,
            path: sm.path.clone(),
            component: sm.component.clone(),
            is_frame: sm.is_frame,
            is_cache: sm.is_cache,
            menu_type: sm.menu_type.clone(),
            visible: sm.visible,
            status: sm.status,
            perms: sm.perms.clone(),
            icon: sm.icon.clone(),
            remark: sm.remark.clone(),
            parent_ids: sm.parent_ids.clone(),
            tree_sort: sm.tree_sort,
            tree_sorts: sm.tree_sorts.clone(),
            tree_level: sm.tree_level,
            tree_leaf: sm.tree_leaf,
            create_by: sm.create_by,
            create_at: sm.create_at,
            update_by: sm.update_by,
            update_at: sm.update_at,
            del_flag: sm.del_flag,
        }
    }
}
