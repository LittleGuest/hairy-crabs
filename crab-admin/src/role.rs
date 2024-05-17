use crab_common::result::Res;
use crab_model::{SysRole, SysRoleReq};
use crab_service::SRV;
use poem::{handler, web::Json, IntoResponse};

#[handler]
pub async fn list() -> impl IntoResponse {
    Res::from(SRV.role.list().await)
}

#[handler]
pub async fn page(Json(req): Json<SysRoleReq>) -> impl IntoResponse {
    Res::from(SRV.role.page(req).await)
}

#[handler]
pub async fn get_by_id(Json(req): Json<SysRoleReq>) -> impl IntoResponse {
    Res::from(SRV.role.get_by_id(req).await)
}

#[handler]
pub async fn save(Json(req): Json<SysRole>) -> impl IntoResponse {
    Res::from(SRV.role.save(req).await)
}

#[handler]
pub async fn update(Json(req): Json<SysRole>) -> impl IntoResponse {
    Res::from(SRV.role.update(req).await)
}

#[handler]
pub async fn delete(Json(req): Json<SysRoleReq>) -> impl IntoResponse {
    Res::from(SRV.role.delete(req).await)
}

#[handler]
pub async fn delete_batch(Json(ids): Json<Vec<i64>>) -> impl IntoResponse {
    Res::from(SRV.role.delete_batch(&ids).await)
}

#[handler]
pub async fn refresh_cache() -> impl IntoResponse {
    Res::from(SRV.role.refresh_cache().await)
}

#[handler]
pub async fn export() -> impl IntoResponse {
    todo!("导出excel")
}
