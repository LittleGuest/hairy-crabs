//!

use jsonwebtoken::{errors::ErrorKind, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use self::error::CrabError;

pub mod cache;
pub mod consts;
pub mod enums;
pub mod error;
pub mod utils;

/// JWT 鉴权 Token结构
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JWTToken {
    //账号id
    pub id: String,
    //账号
    pub account: String,
    //权限集合
    pub permissions: Vec<String>,
    //角色id集合
    pub role_ids: Vec<String>,
    //过期时间
    pub exp: usize,
}

impl JWTToken {
    /// 生成token
    pub fn create_token(&self, secret: &str) -> Result<String, CrabError> {
        return match jsonwebtoken::encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret.as_ref()),
        ) {
            Ok(t) => Ok(t),
            Err(_) => Err(CrabError::JwtError("JWTToken encode fail!")),
        };
    }
    /// 验证token是否有效
    pub fn verify(secret: &str, token: &str) -> Result<JWTToken, CrabError> {
        let validation = Validation {
            ..Validation::default()
        };
        return match jsonwebtoken::decode::<JWTToken>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &validation,
        ) {
            Ok(c) => Ok(c.claims),
            Err(err) => match *err.kind() {
                ErrorKind::InvalidToken => return Err(CrabError::JwtError("InvalidToken")),
                ErrorKind::InvalidIssuer => return Err(CrabError::JwtError("InvalidIssuer")),
                _ => return Err(CrabError::JwtError("InvalidToken other errors")),
            },
        };
    }
}

// /// 返回状态码
// /// 操作成功
// pub const Status_SUCCESS: u32 = 200;
// /// 对象创建成功
// pub const Status_CREATED: u32 = 201;
// /// 请求已经被接受
// pub const Status_ACCEPTED: u32 = 202;
// /// 操作已经执行成功，但是没有返回数据
// pub const Status_NO_CONTENT: u32 = 204;
// /// 资源已被移除
// pub const Status_MOVED_PERM: u32 = 301;
// /// 重定向
// pub const Status_SEE_OTHER: u32 = 303;
// /// 资源没有被修改
// pub const Status_NOT_MODIFIED: u32 = 304;
// /// 参数列表错误（缺少，格式不匹配）
// pub const Status_BAD_REQUEST: u32 = 400;
// /// 未授权
// pub const Status_UNAUTHORIZED: u32 = 401;
// /// 访问受限，授权过期
// pub const Status_FORBIDDEN: u32 = 403;
// /// 资源，服务未找到
// pub const Status_NOT_FOUND: u32 = 404;
// /// 不允许的http方法
// pub const Status_BAD_METHOD: u32 = 405;
// /// 资源冲突，或者资源被锁
// pub const Status_CONFLICT: u32 = 409;
// /// 不支持的数据，媒体类型
// pub const Status_UNSUPPORTED_TYPE: u32 = 415;
// /// 系统内部错误
// pub const Status_ERROR: u32 = 500;
// /// 接口未实现
// pub const Status_NOT_IMPLEMENTED: u32 = 501;
// /// 失败标记
// pub const Status_FAIL: u32 = 500;
