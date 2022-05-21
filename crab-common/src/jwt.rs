use crab_config::APP;
use crab_lib::{
    jsonwebtoken::{self, errors::ErrorKind, DecodingKey, EncodingKey, Header, Validation},
    log,
};
use serde::{Deserialize, Serialize};

use crate::error::CrabError;

/// JWT 鉴权 Token结构
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenData {
    //过期时间
    pub exp: u128,

    //用户ID
    pub user_id: i64,
    //账号
    pub account: String,
    //权限集合
    pub permissions: Vec<String>,
    //角色id集合
    pub role_ids: Vec<String>,
}

impl TokenData {
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
    pub fn verify(token: &str) -> Result<TokenData, CrabError> {
        return match jsonwebtoken::decode::<TokenData>(
            token,
            &DecodingKey::from_secret(APP.jwt_secret.as_bytes()),
            &Validation::default(),
        ) {
            Ok(c) => Ok(c.claims),
            Err(err) => match *err.kind() {
                ErrorKind::InvalidToken => return Err(CrabError::JwtError("InvalidToken")),
                ErrorKind::InvalidIssuer => return Err(CrabError::JwtError("InvalidIssuer")),
                _ => {
                    log::error!("InvalidToken other errors: {err}");
                    return Err(CrabError::JwtError("InvalidToken other errors"));
                }
            },
        };
    }
}

#[cfg(test)]
mod test {
    use super::TokenData;

    #[test]
    fn encode() {
        let td = TokenData {
            exp: 1,
            user_id: 1,
            account: "account".to_string(),
            permissions: vec![],
            role_ids: vec![],
        };
        assert_eq!("", td.create_token().unwrap());
    }

    #[test]
    fn verify() {
        let td = TokenData {
            exp: 1,
            user_id: 1,
            account: "account".to_string(),
            permissions: vec![],
            role_ids: vec![],
        };
        let token = td.create_token().unwrap();
        assert_eq!(td, TokenData::verify(&token).unwrap());
    }
}
