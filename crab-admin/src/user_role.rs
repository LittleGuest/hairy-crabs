use crab_common::result::Res;
use crab_model::SysUserRole;
use crab_service::SRV;
use poem::{handler, web::Json, IntoResponse};

#[handler]
pub async fn save_batch(Json(urs): Json<Vec<SysUserRole>>) -> impl IntoResponse {
    Res::from(SRV.user_role.save_batch(&urs).await)
}

#[handler]
pub async fn delete(Json(ur): Json<SysUserRole>) -> impl IntoResponse {
    Res::from(SRV.user_role.delete(ur).await)
}
