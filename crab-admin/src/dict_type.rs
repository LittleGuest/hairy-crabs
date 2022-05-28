use crab_common::result::Res;
use crab_model::{SysDictType, SysDictTypeReq};
use crab_service::SRV;
use poem::{handler, web::Json, IntoResponse};

#[handler]
pub async fn list() -> impl IntoResponse {
    Res::from(SRV.dict_type.list().await)
}

#[handler]
pub async fn page(Json(req): Json<SysDictTypeReq>) -> impl IntoResponse {
    Res::from(SRV.dict_type.page(req).await)
}

#[handler]
pub async fn get_by_id(Json(req): Json<SysDictTypeReq>) -> impl IntoResponse {
    Res::from(SRV.dict_type.get_by_id(req).await)
}

#[handler]
pub async fn save(Json(req): Json<SysDictType>) -> impl IntoResponse {
    Res::from(SRV.dict_type.save(req).await)
}

#[handler]
pub async fn update(Json(req): Json<SysDictType>) -> impl IntoResponse {
    Res::from(SRV.dict_type.update(req).await)
}

#[handler]
pub async fn delete(Json(req): Json<SysDictTypeReq>) -> impl IntoResponse {
    Res::from(SRV.dict_type.delete(req).await)
}

#[handler]
pub async fn delete_batch(Json(ids): Json<Vec<i64>>) -> impl IntoResponse {
    Res::from(SRV.dict_type.delete_batch(&ids).await)
}

#[handler]
pub async fn refresh_cache() -> impl IntoResponse {
    Res::from(SRV.dict_type.refresh_cache().await)
}

#[handler]
pub async fn export() -> impl IntoResponse {
    todo!("导出excel")
}
