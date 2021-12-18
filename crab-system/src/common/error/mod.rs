//! 全局异常

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CrabError {
    #[error("{0}")]
    E(&'static str),
    #[error("序列化错误")]
    SerializeError,
    #[error("验证码错误")]
    CaptchaError,
    #[error("验证码失效")]
    CaptchaExpireError,
    #[error("账号或密码错误")]
    UsernameOrPasswordError,
    #[error("用户不存在")]
    UserNotFound,
    #[error("{0}")]
    JwtError(&'static str),
    #[error("SQL 错误 : {0}")]
    SQLError(&'static str),
    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
    #[error("未知错误")]
    Unknown,
}
