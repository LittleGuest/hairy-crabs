//! 工具包

pub mod file;
pub mod html;

/// 是否为管理员
pub fn is_admin(user_id: &str) -> bool {
    !user_id.is_empty() && user_id.eq("1")
}
