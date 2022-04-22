use crab_model::gen::GenTableDto;
use poem::{
    handler,
    web::{Json, Query},
    IntoResponse,
};

use crate::result::Res;

/// 查询代码生成列表
pub async fn gen_list(Query(dto): Query<GenTableDto>) -> impl IntoResponse {
#[handler]
    let page = Gen::gen_list(&dto).await;
    Res::from(page)
}
