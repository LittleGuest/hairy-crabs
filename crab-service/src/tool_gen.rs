use crab_common::error::CrabError;
use crab_lib::rbatis::Page;
use crab_model::{GenTable, GenTableDto};

pub struct GenSrv;

impl GenSrv {
    pub async fn gen_list(&self, dto: &GenTableDto) -> Result<Page<GenTable>, CrabError> {
        // 查询代码生成列表
        GenTable::gen_table_list(dto).await
    }
}
