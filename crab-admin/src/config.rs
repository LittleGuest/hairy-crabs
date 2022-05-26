use crab_common::result::Res;
use crab_model::{ConfigReq, SysConfig};
use crab_service::SRV;
use poem::{handler, web::Json, IntoResponse};

#[handler]
pub async fn list() -> impl IntoResponse {
    Res::from(SRV.config.list().await)
}

#[handler]
pub async fn page(Json(req): Json<ConfigReq>) -> impl IntoResponse {
    Res::from(SRV.config.page(req).await)
}

#[handler]
pub async fn get_by_id(Json(req): Json<ConfigReq>) -> impl IntoResponse {
    Res::from(SRV.config.get_by_id(req).await)
}

#[handler]
pub async fn save(Json(req): Json<SysConfig>) -> impl IntoResponse {
    Res::from(SRV.config.save(req).await)
}

#[handler]
pub async fn update(Json(req): Json<SysConfig>) -> impl IntoResponse {
    Res::from(SRV.config.update(req).await)
}

#[handler]
pub async fn delete(Json(req): Json<ConfigReq>) -> impl IntoResponse {
    Res::from(SRV.config.delete(req).await)
}

#[handler]
pub async fn delete_batch(Json(ids): Json<Vec<i64>>) -> impl IntoResponse {
    Res::from(SRV.config.delete_batch(&ids).await)
}

#[handler]
pub async fn refresh_cache() -> impl IntoResponse {
    Res::from(SRV.config.refresh_cache().await)
}

#[handler]
pub async fn export() -> impl IntoResponse {
    todo!("导出excel")
}
