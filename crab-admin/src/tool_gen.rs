use crab_system::{dto::gen::GenTableDto, service::tool_gen::Gen};
use poem::{
    handler,
    web::{Json, Query},
    IntoResponse,
};

use crate::result::Res;

/// 查询代码生成列表
#[handler]
pub async fn gen_list(Query(dto): Query<GenTableDto>) -> impl IntoResponse {
    let page = Gen::gen_list(&dto).await;
    Res::from(page)
}
