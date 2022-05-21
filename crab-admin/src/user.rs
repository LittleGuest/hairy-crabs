use crab_common::{jwt::TokenData, result::Res};
use crab_model::{LoginBody, UserReq};
use crab_service::user::{SysLogin, UserService};
use poem::{
    handler,
    web::{Data, Json},
    IntoResponse,
};

/// 登录
#[handler]
pub async fn login(
    Json(LoginBody {
        account,
        password,
        code,
        uuid,
    }): Json<LoginBody>,
) -> impl IntoResponse {
    Res::from(SysLogin::login(account, password, code, uuid).await)
}

/// 获取登录用户信息
#[handler]
pub async fn user_info(Data(token): Data<&TokenData>) -> impl IntoResponse {
    Res::from(SysLogin::user_info(token.user_id).await)
}

/// 获取登录用户路由信息
#[handler]
pub async fn routers(Data(token): Data<&TokenData>) -> impl IntoResponse {
    Res::from(SysLogin::routers(token.user_id).await)
}

/// 获取用户分页
#[handler]
pub async fn page(Json(req): Json<UserReq>) -> impl IntoResponse {
    Res::from(UserService::page(req).await)
}
