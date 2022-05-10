use std::time::Duration;

use crab_common::error::CrabError;
use crab_lib::log;
use rbatis::{
    crud::CRUD, executor::RbatisExecutor, html_sql, push_index, rb_html, Page, PageRequest,
};

use crate::{dto::gen::GenTableDto, GenTable, RB};

impl GenTable {
    /// 查询业务列表
    pub async fn gen_table_list(
        GenTableDto {
            page_num,
            page_size,
            table_name,
            table_comment,
            ..
        }: &GenTableDto,
    ) -> Result<Page<Self>, CrabError> {
        let mut sql = "
        select
            table_id,
            table_name,
            table_comment,
            sub_table_name,
            sub_table_fk_name,
            class_name,
            tpl_category,
            package_name,
            module_name,
            business_name,
            function_name,
            function_author,
            function_author_email,
            gen_type,
            options,
            create_by,
            create_time,
            update_by,
            update_time,
            remark
        from
            gen_table
        where
            1 = 1
        "
        .to_string();

        if let Some(tn) = table_name {
            sql.push_str(
                format!(" AND lower(table_name) like lower(concat('%', '{tn}', '%')) ").as_str(),
            );
        }

        if let Some(tc) = table_comment {
            sql.push_str(
                format!(" AND lower(table_comment) like lower(concat('%', '{tc}', '%')) ").as_str(),
            );
        }

        let page: Result<Page<Self>, CrabError> = RB
            .fetch_page(&sql, vec![], &PageRequest::new_option(page_num, page_size))
            .await
            .map_err(|e| {
                log::error!("查询业务列表: {}", e);
                CrabError::ServerError("查询业务列表")
            });
        page
    }
}
