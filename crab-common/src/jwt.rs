use crab_config::APP;
use crab_lib::jsonwebtoken::{
    self, errors::ErrorKind, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};

use crate::error::CrabError;

/// JWT 鉴权 Token结构
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JWTToken {
    //用户ID
    pub user_id: i64,
    //账号
    pub account: String,
    //权限集合
    pub permissions: Vec<String>,
    //角色id集合
    pub role_ids: Vec<String>,
    //过期时间
    // pub exp: u128,
}

impl JWTToken {
    /// 生成token
    pub fn create_token(&self) -> Result<String, CrabError> {
        return match jsonwebtoken::encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(APP.jwt_secret.as_bytes()),
        ) {
            Ok(t) => Ok(t),
            Err(_) => Err(CrabError::JwtError("JWTToken encode fail!")),
        };
    }

    /// 验证token是否有效
    pub fn verify(token: &str) -> Result<JWTToken, CrabError> {
        let validation = Validation {
            ..Validation::default()
        };
        return match jsonwebtoken::decode::<JWTToken>(
            token,
            &DecodingKey::from_secret(APP.jwt_secret.as_bytes()),
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
