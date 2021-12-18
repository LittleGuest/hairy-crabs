use serde::{Deserialize, Serialize};

use crate::RB;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all(deserialize = "SCREAMING_SNAKE_CASE"))]
pub struct Table {
    // pub table_catalog: Option<String>,
    pub table_schema: Option<String>,
    pub table_name: Option<String>,
    // pub table_type: Option<String>,
    // pub engine: Option<String>,
    // pub version: Option<String>,
    // pub row_format: Option<String>,
    // pub table_rows: Option<String>,
    // pub avg_row_length: Option<String>,
    // pub data_length: Option<String>,
    // pub max_data_length: Option<String>,
    // pub index_length: Option<String>,
    // pub data_free: Option<String>,
    // pub auto_increment: Option<String>,
    // pub create_time: Option<NaiveDateTime>,
    // pub update_time: Option<NaiveDateTime>,
    // pub check_time: Option<NaiveDateTime>,
    // pub table_collation: Option<String>,
    // pub checksum: Option<String>,
    // pub create_options: Option<String>,
    pub table_comment: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all(deserialize = "SCREAMING_SNAKE_CASE"))]
pub struct TableColumn {
    // pub table_catalog: Option<String>,
    pub table_schema: Option<String>,
    pub table_name: Option<String>,
    pub column_name: Option<String>,
    pub ordinal_position: Option<u8>,
    pub column_default: Option<String>,
    pub is_nullable: Option<String>,
    pub data_type: Option<String>,
    pub character_maximum_length: Option<u64>,
    // pub character_octet_length: Option<String>,
    // pub numeric_precision: Option<String>,
    // pub numeric_scale: Option<String>,
    // pub datetime_precision: Option<String>,
    // pub character_set_name: Option<String>,
    // pub collation_name: Option<String>,
    pub column_type: Option<String>,
    pub column_key: Option<String>,
    // pub extra: Option<String>,
    // pub privileges: Option<String>,
    pub column_comment: Option<String>,
    // pub generation_expression: Option<String>,
    // pub srs_id: Option<String>,

    // 对应 Rust 类型
    pub field_type: Option<String>,
    pub multi_world: Option<bool>,
    pub r#type: Option<String>,
}

/// 获取指定表信息
///
/// ```text
/// table_names：表名，英文逗号拼接
/// ```
#[py_sql(
    RB,
    "
    SELECT
        TABLE_CATALOG,
        TABLE_SCHEMA,
        TABLE_NAME,
        TABLE_TYPE,
        `ENGINE`,
        VERSION,
        ROW_FORMAT,
        TABLE_ROWS,
        AVG_ROW_LENGTH,
        DATA_LENGTH,
        MAX_DATA_LENGTH,
        INDEX_LENGTH,
        DATA_FREE,
        AUTO_INCREMENT,
        CREATE_TIME,
        UPDATE_TIME,
        CHECK_TIME,
        TABLE_COLLATION,
        `CHECKSUM`,
        CREATE_OPTIONS,
        TABLE_COMMENT
    FROM
        information_schema.`TABLES`
    WHERE
        TABLE_SCHEMA = (
        SELECT
            DATABASE ())
    if table_names != '':
        AND FIND_IN_SET(TABLE_NAME, #{table_names})
    ORDER BY
        CREATE_TIME
    "
)]
pub(crate) async fn tables(table_names: &str) -> Vec<Table> {}

/// 获取指定表的列信息
///
/// ```text
/// table_names：表名，英文逗号拼接
/// ```
#[py_sql(
    RB,
    "
    SELECT
        TABLE_CATALOG,
        TABLE_SCHEMA,
        TABLE_NAME,
        COLUMN_NAME,
        ORDINAL_POSITION,
        COLUMN_DEFAULT,
        IS_NULLABLE,
        DATA_TYPE,
        CHARACTER_MAXIMUM_LENGTH,
        CHARACTER_OCTET_LENGTH,
        NUMERIC_PRECISION,
        NUMERIC_SCALE,
        DATETIME_PRECISION,
        CHARACTER_SET_NAME,
        COLLATION_NAME,
        COLUMN_TYPE,
        COLUMN_KEY,
        EXTRA,
        `PRIVILEGES`,
        COLUMN_COMMENT,
        GENERATION_EXPRESSION,
        SRS_ID
    FROM
        information_schema.COLUMNS
    WHERE
        TABLE_SCHEMA = (
        SELECT
            DATABASE ())
    if table_names != '':
        AND FIND_IN_SET(TABLE_NAME, #{table_names})
    ORDER BY
        ORDINAL_POSITION
    "
)]
pub(crate) async fn tables_columns(table_names: &str) -> Vec<TableColumn> {}
