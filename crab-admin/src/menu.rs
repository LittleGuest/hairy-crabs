use crab_common::result::Res;
use crab_model::{MenuReq, SysMenu};
use crab_service::SRV;
use poem::{handler, web::Json, IntoResponse};

#[handler]
pub async fn list() -> impl IntoResponse {
    Res::from(SRV.menu.list().await)
}

#[handler]
pub async fn page(Json(req): Json<MenuReq>) -> impl IntoResponse {
    Res::from(SRV.menu.page(req).await)
}

#[handler]
pub async fn get_by_id(Json(req): Json<MenuReq>) -> impl IntoResponse {
    Res::from(SRV.menu.get_by_id(req).await)
}

#[handler]
pub async fn save(Json(req): Json<SysMenu>) -> impl IntoResponse {
    Res::from(SRV.menu.save(req).await)
}

#[handler]
pub async fn update(Json(req): Json<SysMenu>) -> impl IntoResponse {
    Res::from(SRV.menu.update(req).await)
}

#[handler]
pub async fn delete(Json(req): Json<MenuReq>) -> impl IntoResponse {
    Res::from(SRV.menu.delete(req).await)
}

#[handler]
pub async fn delete_batch(Json(ids): Json<Vec<i64>>) -> impl IntoResponse {
    Res::from(SRV.menu.delete_batch(&ids).await)
}

#[handler]
pub async fn refresh_cache() -> impl IntoResponse {
    Res::from(SRV.menu.refresh_cache().await)
}

#[handler]
pub async fn export() -> impl IntoResponse {
    todo!("导出excel")
}
