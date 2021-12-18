//! 前端传过来的请求参数

pub mod gen;
pub mod menu_dto;

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::model::SysUser;

/// 用户登录
#[derive(Serialize, Deserialize)]
pub struct LoginBody {
    /// 用户名
    pub username: String,
    /// 密码
    pub password: String,
    /// 验证码
    pub code: String,
    /// 唯一标识
    pub uuid: String,
}

#[derive(Serialize)]
pub struct LoginUserDto {
    /// 用户信息
    pub user: SysUser,
    /// jwt
    pub access_token: String,
}

#[derive(Serialize)]
pub struct UserInfoDto {
    /// 用户信息
    pub user: SysUser,
    /// 角色集合
    pub roles: HashSet<String>,
    /// 权限集合
    pub permissions: HashSet<String>,
    /// license
    pub lincense_info: String,
}
