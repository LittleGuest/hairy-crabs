use rbatis::Page;

use crate::{common::error::CrabError, dto::gen::GenTableDto, model::GenTable};

pub struct Gen;

impl Gen {
    /// 查询代码生成列表
    pub async fn gen_list(dto: &GenTableDto) -> Result<Page<GenTable>, CrabError> {
        GenTable::gen_table_list(dto).await
    }
}
