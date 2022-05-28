use crab_common::result::Res;
use crab_model::{GenTable, GenTableReq};
use crab_service::SRV;
use poem::{handler, web::Json, IntoResponse};

#[handler]
pub async fn list() -> impl IntoResponse {
    Res::from(SRV.gen_table.list().await)
}

#[handler]
pub async fn page(Json(req): Json<GenTableReq>) -> impl IntoResponse {
    Res::from(SRV.gen_table.page(req).await)
}

#[handler]
pub async fn get_by_id(Json(req): Json<GenTableReq>) -> impl IntoResponse {
    Res::from(SRV.gen_table.get_by_id(req).await)
}

#[handler]
pub async fn save(Json(req): Json<GenTable>) -> impl IntoResponse {
    Res::from(SRV.gen_table.save(req).await)
}

#[handler]
pub async fn update(Json(req): Json<GenTable>) -> impl IntoResponse {
    Res::from(SRV.gen_table.update(req).await)
}

#[handler]
pub async fn delete(Json(req): Json<GenTableReq>) -> impl IntoResponse {
    Res::from(SRV.gen_table.delete(req).await)
}

#[handler]
pub async fn delete_batch(Json(ids): Json<Vec<i64>>) -> impl IntoResponse {
    Res::from(SRV.gen_table.delete_batch(&ids).await)
}
