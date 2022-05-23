use crab_common::{jwt::TokenData, result::Res};
use crab_model::{LoginBody, SysUser, UserReq};
use crab_service::SRV;
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
    Res::from(SRV.login.login(account, password, code, uuid).await)
}

/// 获取登录用户信息
#[handler]
pub async fn user_info(Data(token): Data<&TokenData>) -> impl IntoResponse {
    Res::from(SRV.login.user_info(token.user_id).await)
}

/// 获取登录用户路由信息
#[handler]
pub async fn routers(Data(token): Data<&TokenData>) -> impl IntoResponse {
    Res::from(SRV.login.routers(token.user_id).await)
}

/// 获取用户分页
#[handler]
pub async fn page(Json(req): Json<UserReq>) -> impl IntoResponse {
    Res::from(SRV.user.page(req).await)
}

/// 新增用户
#[handler]
pub async fn save(Json(user): Json<SysUser>) -> impl IntoResponse {
    Res::from(SRV.user.save(user).await)
}

/// 编辑用户
#[handler]
pub async fn update(Json(user): Json<SysUser>) -> impl IntoResponse {
    Res::from(SRV.user.update(user).await)
}

/// 批量编辑用户
#[handler]
pub async fn update_batch(Json(users): Json<Vec<SysUser>>) -> impl IntoResponse {
    Res::from(SRV.user.update_batch(&users).await)
}

/// 删除用户
#[handler]
pub async fn delete(Json(user): Json<SysUser>) -> impl IntoResponse {
    Res::from(SRV.user.delete(user).await)
}

/// 批量删除用户
#[handler]
pub async fn delete_batch(Json(ids): Json<Vec<i64>>) -> impl IntoResponse {
    Res::from(SRV.user.delete_batch(&ids).await)
}
