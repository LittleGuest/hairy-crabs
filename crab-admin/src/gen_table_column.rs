use crab_common::result::Res;
use crab_model::{GenTableColumn, GenTableColumnReq};
use crab_service::SRV;
use poem::{handler, web::Json, IntoResponse};

#[handler]
pub async fn list() -> impl IntoResponse {
    Res::from(SRV.gen_table_column.list().await)
}

#[handler]
pub async fn page(Json(req): Json<GenTableColumnReq>) -> impl IntoResponse {
    Res::from(SRV.gen_table_column.page(req).await)
}

#[handler]
pub async fn get_by_id(Json(req): Json<GenTableColumnReq>) -> impl IntoResponse {
    Res::from(SRV.gen_table_column.get_by_id(req).await)
}

#[handler]
pub async fn save(Json(req): Json<GenTableColumn>) -> impl IntoResponse {
    Res::from(SRV.gen_table_column.save(req).await)
}

#[handler]
pub async fn update(Json(req): Json<GenTableColumn>) -> impl IntoResponse {
    Res::from(SRV.gen_table_column.update(req).await)
}

#[handler]
pub async fn delete(Json(req): Json<GenTableColumnReq>) -> impl IntoResponse {
    Res::from(SRV.gen_table_column.delete(req).await)
}

#[handler]
pub async fn delete_batch(Json(ids): Json<Vec<i64>>) -> impl IntoResponse {
    Res::from(SRV.gen_table_column.delete_batch(&ids).await)
}
