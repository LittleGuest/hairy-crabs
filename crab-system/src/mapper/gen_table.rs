use rbatis::{crud::CRUD, Page, PageRequest};

use crate::{common::error::CrabError, dto::gen::GenTableDto, model::GenTable, RB};

impl GenTable {
    /// 查询业务列表
    pub async fn gen_table_list(dto: &GenTableDto) -> Result<Page<Self>, CrabError> {
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

        if dto.table_name.is_some() {
            sql.push_str("\nAND lower(table_name) like lower(concat('%', #{table_name}, '%'))");
        }
        if dto.table_comment.is_some() {
            sql.push_str(
                "\nAND lower(table_comment) like lower(concat('%', #{table_comment}, '%'))",
            );
        }
        let page: Result<Page<Self>, CrabError> = RB
            .fetch_page(
                &sql,
                vec![as_bson!(&dto.table_name), as_bson!(&dto.table_comment)],
                &PageRequest::new_option(&dto.page_num, &dto.page_size),
            )
            .await
            .map_err(|e| CrabError::SQLError("查询业务列表"));
        page
    }
}
