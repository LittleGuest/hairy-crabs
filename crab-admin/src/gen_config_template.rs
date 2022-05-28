use crab_common::result::Res;
use crab_model::{GenConfigTemplate, GenConfigTemplateReq};
use crab_service::SRV;
use poem::{handler, web::Json, IntoResponse};

#[handler]
pub async fn list() -> impl IntoResponse {
    Res::from(SRV.gen_config_template.list().await)
}

#[handler]
pub async fn page(Json(req): Json<GenConfigTemplateReq>) -> impl IntoResponse {
    Res::from(SRV.gen_config_template.page(req).await)
}

#[handler]
pub async fn get_by_id(Json(req): Json<GenConfigTemplateReq>) -> impl IntoResponse {
    Res::from(SRV.gen_config_template.get_by_id(req).await)
}

#[handler]
pub async fn save(Json(req): Json<GenConfigTemplate>) -> impl IntoResponse {
    Res::from(SRV.gen_config_template.save(req).await)
}

#[handler]
pub async fn update(Json(req): Json<GenConfigTemplate>) -> impl IntoResponse {
    Res::from(SRV.gen_config_template.update(req).await)
}

#[handler]
pub async fn delete(Json(req): Json<GenConfigTemplateReq>) -> impl IntoResponse {
    Res::from(SRV.gen_config_template.delete(req).await)
}

#[handler]
pub async fn delete_batch(Json(ids): Json<Vec<i64>>) -> impl IntoResponse {
    Res::from(SRV.gen_config_template.delete_batch(&ids).await)
}

#[handler]
pub async fn change_status(Json(_gct): Json<GenConfigTemplate>) -> impl IntoResponse {
    todo!()
}

#[handler]
pub async fn change_template_default(Json(_gct): Json<GenConfigTemplate>) -> impl IntoResponse {
    todo!()
}
