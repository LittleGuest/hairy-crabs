use crab_system::{dto::LoginBody, service::sys_auth::SysLogin};
use poem::{
    handler,
    web::{Json, Path},
    IntoResponse,
};

use crate::result::Res;

/// 登录
#[handler]
pub async fn login(
    Json(LoginBody {
        username,
        password,
        code,
        uuid,
    }): Json<LoginBody>,
) -> impl IntoResponse {
    let login = SysLogin::login(username, password, code, uuid).await;
    Res::from(login)
}

/// 获取用户信息
#[handler]
pub async fn user_info(Path(user_id): Path<String>) -> impl IntoResponse {
    let user_info = SysLogin::user_info(&user_id).await;
    Res::from(user_info)
}

/// 获取用户路由信息
#[handler]
pub async fn routers(Path(user_id): Path<String>) -> impl IntoResponse {
    let routers = SysLogin::routers(&user_id).await;
    Res::from(routers)
}
