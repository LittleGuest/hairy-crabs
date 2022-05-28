use crab_common::{jwt::TokenData, result::Res};
use crab_model::{LoginBody, ResetPwdReq, SysUser, SysUserReq};
use crab_service::SRV;
use poem::{
    handler,
    web::{Data, Json},
    IntoResponse,
};

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

#[handler]
pub async fn user_info(Data(token): Data<&TokenData>) -> impl IntoResponse {
    Res::from(SRV.login.user_info(token.user_id).await)
}

#[handler]
pub async fn routers(Data(token): Data<&TokenData>) -> impl IntoResponse {
    Res::from(SRV.login.routers(token.user_id).await)
}

#[handler]
pub async fn page(Json(req): Json<SysUserReq>) -> impl IntoResponse {
    Res::from(SRV.user.page(req).await)
}

#[handler]
pub async fn save(Json(user): Json<SysUser>) -> impl IntoResponse {
    Res::from(SRV.user.save(user).await)
}

#[handler]
pub async fn update(Json(user): Json<SysUser>) -> impl IntoResponse {
    Res::from(SRV.user.update(user).await)
}

#[handler]
pub async fn update_batch(Json(users): Json<Vec<SysUser>>) -> impl IntoResponse {
    Res::from(SRV.user.update_batch(&users).await)
}

#[handler]
pub async fn delete(Json(user): Json<SysUser>) -> impl IntoResponse {
    Res::from(SRV.user.delete(user).await)
}

#[handler]
pub async fn delete_batch(Json(ids): Json<Vec<i64>>) -> impl IntoResponse {
    Res::from(SRV.user.delete_batch(&ids).await)
}

#[handler]
pub async fn reset_pwd(Json(req): Json<ResetPwdReq>) -> impl IntoResponse {
    Res::from(SRV.user.reset_pwd(req).await)
}

#[handler]
pub async fn import() -> impl IntoResponse {
    todo!()
}

#[handler]
pub async fn export() -> impl IntoResponse {
    todo!()
}
