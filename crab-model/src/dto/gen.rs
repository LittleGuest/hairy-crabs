use serde::Deserialize;

#[derive(Deserialize)]
pub struct GenTableDto {
    pub page_num: Option<u64>,
    pub page_size: Option<u64>,
    pub table_name: Option<String>,
    pub table_comment: Option<String>,
    pub function_author: Option<String>,
}
