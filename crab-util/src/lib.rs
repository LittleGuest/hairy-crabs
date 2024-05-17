//! 工具包

#![allow(unused)]

use std::time::SystemTime;

pub mod file;
pub mod html;
pub mod password_encoder;

/// 获取当前时间，毫秒
pub fn now_millis() -> u128 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
