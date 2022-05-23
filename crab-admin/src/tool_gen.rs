use crab_common::result::Res;
use crab_lib::poem::{handler, web::Query, IntoResponse};
use crab_model::GenTableDto;
use crab_service::SRV;

/// 查询代码生成列表
#[handler]
pub async fn gen_list(Query(dto): Query<GenTableDto>) -> impl IntoResponse {
    let page = SRV.gen.gen_list(&dto).await;
    Res::from(page)
}
