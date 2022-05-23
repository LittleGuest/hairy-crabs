//! 登录验证

use std::collections::HashSet;

use crab_cache::{ConfigUtil, RedisCache};
use crab_common::{consts, error::CrabError, jwt::TokenData, result::CrabResult};
use crab_lib::{
    log,
    rbatis::{snowflake::new_snowflake_id, Page},
    validator::Validate,
};
use crab_model::{Mapper, SysLoginLog, SysMenu, SysMenuTreeDto, SysUser, UserInfoDto, UserReq};
use crab_util::password_encoder::PasswordEncoder;

#[derive(Clone, Copy)]
pub struct SysLoginSrv;

impl SysLoginSrv {
    /// 登录验证
    pub async fn login(
        &self,
        account: String,
        password: String,
        code: String,
        uuid: String,
    ) -> CrabResult<String> {
        // 验证码开关
        if ConfigUtil::get_config_bool_value_by_key(consts::config::SYS_CAPTCHA_ON_OFF, true) {
            Self::verify_captcha(&account, &code, &uuid).await?;
        }

        if let Some(user) = SysUser::get_by_username(&account).await? {
            if !PasswordEncoder::verify(
                &password,
                user.password
                    .as_ref()
                    .ok_or(CrabError::UsernameOrPasswordError)?,
            ) {
                return Err(CrabError::UsernameOrPasswordError);
            }

            let jwt_token = TokenData {
                user_id: user.id.unwrap(),
                account,
                permissions: vec![],
                role_ids: vec![],
                // TODO 时间
                exp: 1111111111111111,
            };

            let login_log = SysLoginLog {
                id: Some(new_snowflake_id()),
                account: user.account,
                ..Default::default()
            };
            login_log.save().await?;

            Ok(jwt_token.create_token()?)
        } else {
            Err(CrabError::UserNotFound)
        }
    }

    /// 获取用户信息
    pub async fn user_info(&self, user_id: i64) -> CrabResult<UserInfoDto> {
        if let Some(mut user) = SysUser::get_by_id(user_id).await? {
            // TODO 获取角色集合
            // TODO 获取权限集合
            user.password = None;

            Ok(UserInfoDto {
                user,
                roles: HashSet::with_capacity(0),
                permissions: HashSet::with_capacity(0),
            })
        } else {
            Err(CrabError::UserNotFound)
        }
    }

    /// 获取用户路由信息
    pub async fn routers(&self, user_id: i64) -> CrabResult<HashSet<SysMenuTreeDto>> {
        let menus = if Self::is_admin(user_id) {
            SysMenu::menus().await?
        } else {
            SysMenu::get_menu_by_user_id(user_id).await?
        };
        Ok(Self::get_child_perms(menus, 0))
    }

    /// 是否为管理员
    pub fn is_admin(user_id: i64) -> bool {
        user_id == 1
    }

    /// 根据父节点的ID获取所有子节点
    fn get_child_perms(menus: HashSet<SysMenu>, pid: i64) -> HashSet<SysMenuTreeDto> {
        let mut menus = menus
            .iter()
            .filter(|m| Some(pid) == m.pid)
            .map(SysMenuTreeDto::from)
            .collect::<Vec<_>>();

        let mut all = HashSet::new();
        let temp = menus.clone();
        for menu in menus.iter_mut() {
            Self::recursion(&temp, menu);
            all.insert(menu.clone());
        }
        all
    }

    /// 递归获取子节点
    fn recursion(menus: &Vec<SysMenuTreeDto>, mt: &mut SysMenuTreeDto) {
        let mut childs = Self::childs(menus, mt);
        mt.has_child = childs.is_empty();
        for child in childs.iter_mut() {
            if mt.has_child {
                Self::recursion(menus, child);
            }
        }
        mt.children = childs;
    }

    /// 得到子节点列表
    fn childs(menus: &[SysMenuTreeDto], mt: &SysMenuTreeDto) -> Vec<SysMenuTreeDto> {
        menus
            .iter()
            .filter(|smt| smt.pid == mt.id)
            .cloned()
            .collect::<Vec<_>>()
    }

    /// 校验验证码
    async fn verify_captcha(account: &str, code: &str, uuid: &str) -> CrabResult<()> {
        let verify_key = format!("{}{}", consts::CAPTCHA_CODE_KEY, uuid);

        let mut login_log = SysLoginLog {
            id: Some(new_snowflake_id()),
            account: Some(account.to_string()),
            ..Default::default()
        };

        if let Some(captcha) = RedisCache::get::<String>(&verify_key) {
            if !code.eq_ignore_ascii_case(&captcha) {
                login_log.msg = Some("验证码错误".to_string());
                login_log.save().await?;
                return Err(CrabError::CaptchaError);
            }
            Ok(())
        } else {
            login_log.msg = Some("验证码失效".to_string());
            login_log.save().await?;
            Err(CrabError::CaptchaExpireError)
        }
    }
}

pub struct UserSrv;

impl UserSrv {
    pub async fn page(&self, req: UserReq) -> CrabResult<Page<SysUser>> {
        let mut page = SysUser::page(&req).await?;
        page.records.iter_mut().for_each(|u| u.password = None);
        Ok(page)
    }

    pub async fn save(&self, mut user: SysUser) -> CrabResult<SysUser> {
        user.validate().map_err(|e| {
            log::error!("validation error: {e}");
            CrabError::ValidationError("")
        })?;
        user.id = user.save().await?;
        Ok(user)
    }

    pub async fn update(&self, user: SysUser) -> CrabResult<SysUser> {
        user.validate().map_err(|e| {
            log::error!("validation error: {e}");
            CrabError::ValidationError("")
        })?;
        user.update().await?;
        Ok(user)
    }

    pub async fn update_batch(&self, users: &[SysUser]) -> CrabResult<u64> {
        for user in users.iter() {
            user.validate().map_err(|e| {
                log::error!("validation error: {e}");
                CrabError::ValidationError("")
            })?;
        }
        SysUser::update_batch(users).await
    }

    pub async fn delete(&self, user: SysUser) -> CrabResult<u64> {
        if let Some(uid) = user.id {
            SysUser::remove_by_id(uid).await
        } else {
            Ok(0)
        }
    }

    pub async fn delete_batch(&self, ids: &[i64]) -> CrabResult<u64> {
        SysUser::remove_batch_by_ids(ids).await
    }
}
