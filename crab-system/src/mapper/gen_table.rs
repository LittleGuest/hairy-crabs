use rbatis::{crud::CRUD, Page, PageRequest};

use crate::{common::error::CrabError, dto::gen::GenTableDto, model::GenTable, RB};

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
        let sql = "
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
        if  table_name != null:
            AND lower(table_name) like lower(concat('%', #{table_name}, '%'))
        if table_comment != null:
            AND lower(table_comment) like lower(concat('%', #{table_comment}, '%'))
        "
        .to_string();

        let page: Result<Page<Self>, CrabError> = RB
            .fetch_page(
                &sql,
                vec![rbson::bson!({ "table_name": table_name,"table_comment": table_comment })],
                &PageRequest::new_option(page_num, page_size),
            )
            .await
            .map_err(|e| {
                log::error!("查询业务列表: {}", e);
                CrabError::SQLError("查询业务列表")
            });
        page
    }
}
