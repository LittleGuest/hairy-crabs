use crab_common::result::Res;
use crab_model::{DictDataReq, SysDictData};
use crab_service::SRV;
use poem::{handler, web::Json, IntoResponse};

#[handler]
pub async fn list() -> impl IntoResponse {
    Res::from(SRV.dict_data.list().await)
}

#[handler]
pub async fn page(Json(req): Json<DictDataReq>) -> impl IntoResponse {
    Res::from(SRV.dict_data.page(req).await)
}

#[handler]
pub async fn get_by_id(Json(req): Json<DictDataReq>) -> impl IntoResponse {
    Res::from(SRV.dict_data.get_by_id(req).await)
}

#[handler]
pub async fn save(Json(req): Json<SysDictData>) -> impl IntoResponse {
    Res::from(SRV.dict_data.save(req).await)
}

#[handler]
pub async fn update(Json(req): Json<SysDictData>) -> impl IntoResponse {
    Res::from(SRV.dict_data.update(req).await)
}

#[handler]
pub async fn delete(Json(req): Json<DictDataReq>) -> impl IntoResponse {
    Res::from(SRV.dict_data.delete(req).await)
}

#[handler]
pub async fn delete_batch(Json(ids): Json<Vec<i64>>) -> impl IntoResponse {
    Res::from(SRV.dict_data.delete_batch(&ids).await)
}

#[handler]
pub async fn refresh_cache() -> impl IntoResponse {
    Res::from(SRV.dict_data.refresh_cache().await)
}

#[handler]
pub async fn get_by_type(Json(req): Json<DictDataReq>) -> impl IntoResponse {
    Res::from(SRV.dict_data.get_by_type(req).await)
}

#[handler]
pub async fn export() -> impl IntoResponse {
    todo!("导出excel")
}
