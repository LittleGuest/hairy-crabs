use crab_common::result::Res;
use crab_model::SysLoginLogReq;
use crab_service::SRV;
use poem::{handler, web::Json, IntoResponse};

#[handler]
pub async fn page(Json(req): Json<SysLoginLogReq>) -> impl IntoResponse {
    Res::from(SRV.login_log.page(req).await)
}

#[handler]
pub async fn get_by_id(Json(req): Json<SysLoginLogReq>) -> impl IntoResponse {
    Res::from(SRV.login_log.get_by_id(req).await)
}

#[handler]
pub async fn clear() -> impl IntoResponse {
    Res::from(SRV.login_log.clear().await)
}

#[handler]
pub async fn export() -> impl IntoResponse {
    todo!("导出excel")
}
