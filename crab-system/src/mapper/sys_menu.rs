use std::collections::HashSet;

use rbatis::executor::Executor;

use crate::{common::error::CrabError, model::SysMenu, RB};

impl SysMenu {
    /// 根据用户ID查询菜单树信息
    pub async fn get_menu_by_user_id(_user_id: &str) -> Result<HashSet<Self>, CrabError> {
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
            t.parent_id ,
            t.parent_ids ,
            t.tree_sort ,
            t.tree_sorts ,
            t.tree_level ,
            t.tree_leaf ,
            t.create_by ,
            t.create_dept ,
            t.create_time ,
            t.update_by ,
            t.update_time ,
            t.update_ip ,
            t.remark ,
            t.version ,
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
            .map_err(|_e| CrabError::SQLError("根据用户ID查询菜单树信息"));
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
            t.parent_id ,
            t.parent_ids ,
            t.tree_sort ,
            t.tree_sorts ,
            t.tree_level ,
            t.tree_leaf ,
            t.create_by ,
            t.create_dept ,
            t.create_time ,
            t.update_by ,
            t.update_time ,
            t.update_ip ,
            t.remark ,
            t.version ,
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
            .map_err(|_e| CrabError::SQLError("根据用户ID查询菜单树信息"));
        menus
    }
}
