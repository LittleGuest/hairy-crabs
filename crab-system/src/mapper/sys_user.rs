use rbatis::crud::CRUD;

use crate::{common::error::CrabError, model::SysUser, RB};
impl SysUser {
    pub async fn get_by_username(username: &str) -> Result<Option<Self>, CrabError> {
        let user: Result<Option<Self>, CrabError> = RB
            .fetch_by_column("user_name", username)
            .await
            .map_err(|_e| CrabError::SQLError("根据用户名获取用户信息"));
        user
    }

    pub async fn get_by_id(user_id: &str) -> Result<Option<Self>, CrabError> {
        let user: Result<Option<Self>, CrabError> = RB
            .fetch_by_column("id", user_id)
            .await
            .map_err(|_e| CrabError::SQLError("根据用户ID获取用户信息"));
        user
    }
}
