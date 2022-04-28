//! 登录验证

use std::{collections::HashSet, time::SystemTime};

use crab_common::{error::CrabError, jwt::JWTToken};
use crab_config::APP;
use crab_model::{menu_dto::SysMenuTreeDto, LoginUserDto, SysMenu, SysUser, UserInfoDto};
use crab_util::password_encoder::PasswordEncoder;

#[derive(Clone, Copy)]
pub struct SysLogin;

impl SysLogin {
    /// 登录验证
    pub async fn login(
        username: String,
        password: String,
        _code: String,
        _uuid: String,
    ) -> Result<LoginUserDto, CrabError> {
        // TODO 验证码开关是否打开
        // let captcha_on =
        //     ConfigUtil::get_config_bool_value_by_key(consts::config::SYS_CAPTCHA_ON_OFF, false);
        // if captcha_on {
        //     Self::validate_captcha(&username, &code, &uuid)?;
        // }

        // 用户验证
        let user = SysUser::get_by_username(&username).await?;
        match user {
            Some(user) => {
                if !PasswordEncoder::verify(
                    user.password
                        .as_ref()
                        .ok_or_else(|| CrabError::UsernameOrPasswordError)?,
                    &password,
                ) {
                    return Err(CrabError::UsernameOrPasswordError);
                }

                let jwt_token = JWTToken {
                    id: user.id.clone().unwrap_or(String::new()),
                    account: username,
                    permissions: vec![],
                    role_ids: vec![],
                    // TODO 时间
                    exp: SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_millis(), // exp: DateTimeNative::now().timestamp_millis() as usize,
                };
                let access_token = jwt_token.create_token(APP.jwt_secret.as_str())?;

                // TODO 记录登录日志

                Ok(LoginUserDto {
                    user,
                    access_token: access_token,
                })
            }
            None => Err(CrabError::UserNotFound),
        }
    }

    /// 获取用户信息
    pub async fn user_info(user_id: &str) -> Result<UserInfoDto, CrabError> {
        // 获取基本信息
        let user = SysUser::get_by_id(user_id).await?;

        if let Some(user) = user {
            // 获取角色集合
            // 获取权限集合
            // 获取用户自定义首页
            // TODO 获取用户待读通知公告
            return Ok(UserInfoDto {
                user,
                roles: HashSet::with_capacity(0),
                permissions: HashSet::with_capacity(0),
                lincense_info: "".to_string(),
            });
        }
        Err(CrabError::UserNotFound)
    }

    /// 获取用户路由信息
    pub async fn routers(user_id: &str) -> Result<HashSet<SysMenuTreeDto>, CrabError> {
        let menus;
        if Self::is_admin(user_id) {
            menus = SysMenu::menus().await?;
        } else {
            menus = SysMenu::get_menu_by_user_id(user_id).await?;
        }
        Ok(Self::get_child_perms(menus, "0"))
    }

    /// 是否为管理员
    pub fn is_admin(user_id: &str) -> bool {
        !user_id.is_empty() && user_id.eq("1")
    }

    /// 根据父节点的ID获取所有子节点
    fn get_child_perms(menus: HashSet<SysMenu>, parent_id: &str) -> HashSet<SysMenuTreeDto> {
        let mut menus = menus
            .iter()
            .filter(|m| Some(parent_id.to_string()).eq(&m.parent_id))
            .map(|m| SysMenuTreeDto::from(m))
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
    fn childs(menus: &Vec<SysMenuTreeDto>, mt: &SysMenuTreeDto) -> Vec<SysMenuTreeDto> {
        menus
            .iter()
            .filter(|smt| smt.parent_id.eq(&mt.id))
            .cloned()
            .collect::<Vec<_>>()
    }

    // /// 校验验证码
    // fn validate_captcha(_username: &str, code: &str, uuid: &str) -> Result<(), CrabError> {
    //     let verify_key = format!("{}{}", consts::CAPTCHA_CODE_KEY, uuid);
    //     let captcha = redis_cache::get::<String>(&verify_key);
    //     match captcha {
    //         Some(captcha) => {
    //             if !code.eq_ignore_ascii_case(&captcha) {
    //                 // TODO 记录登录日志
    //                 return Err(CrabError::CaptchaError);
    //             }

    //             Ok(())
    //         }
    //         None => {
    //             // TODO 记录登录日志
    //             Err(CrabError::CaptchaExpireError)
    //         }
    //     }
    // }
}
