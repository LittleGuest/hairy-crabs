use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{column_keywords, Database, RB};

pub struct Mysql;

#[async_trait]
impl Database for Mysql {
    async fn tables(&self, table_names: &[&str]) -> Vec<crate::Table> {
        let ts = tables(&table_names.join(",")).await.unwrap();
        ts.iter().map(|t| t.into()).collect::<Vec<_>>()
    }
    async fn columns(&self, table_names: &[&str]) -> Vec<crate::TableColumn> {
        let cols = columns(&table_names.join(",")).await.unwrap();
        cols.iter().map(|col| col.into()).collect::<Vec<_>>()
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all(deserialize = "SCREAMING_SNAKE_CASE"))]
pub struct Table {
    pub table_catalog: Option<String>,
    pub table_schema: Option<String>,
    pub table_name: Option<String>,
    /// enum('BASE TABLE','VIEW','SYSTEM VIEW')
    pub table_type: Option<String>,
    pub engine: Option<String>,
    pub version: Option<u64>,
    /// enum('Fixed','Dynamic','Compressed','Redundant','Compact','Paged')
    pub row_format: Option<String>,
    pub table_rows: Option<u64>,
    pub avg_row_length: Option<u64>,
    pub data_length: Option<u64>,
    pub max_data_length: Option<u64>,
    pub index_length: Option<u64>,
    pub data_free: Option<u64>,
    pub auto_increment: Option<u64>,
    // pub create_time: Option<u64>,
    // pub update_time: Option<u64>,
    // pub check_time: Option<u64>,
    pub table_collation: Option<String>,
    pub checksum: Option<i64>,
    pub create_options: Option<String>,
    pub table_comment: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all(deserialize = "SCREAMING_SNAKE_CASE"))]
pub struct TableColumn {
    pub table_catalog: Option<String>,
    pub table_schema: Option<String>,
    pub table_name: Option<String>,
    pub column_name: Option<String>,
    pub ordinal_position: Option<u32>,
    pub column_default: Option<String>,
    pub is_nullable: Option<String>,
    pub data_type: Option<String>,
    pub character_maximum_length: Option<i64>,
    pub character_octet_length: Option<i64>,
    pub numeric_precision: Option<u64>,
    pub numeric_scale: Option<u64>,
    pub datetime_precision: Option<u32>,
    pub character_set_name: Option<String>,
    pub collation_name: Option<String>,
    pub column_type: Option<String>,
    /// enum('','PRI','UNI','MUL')
    pub column_key: Option<String>,
    pub extra: Option<String>,
    pub privileges: Option<String>,
    pub column_comment: Option<String>,
    pub generation_expression: Option<String>,
    pub srs_id: Option<u32>,
}

impl From<Table> for crate::Table {
    fn from(t: Table) -> Self {
        Self {
            schema: t.table_schema,
            name: t.table_name,
            comment: t.table_comment,
        }
    }
}

impl From<&Table> for crate::Table {
    fn from(t: &Table) -> Self {
        Self {
            schema: t.table_schema.clone(),
            name: t.table_name.clone(),
            comment: t.table_comment.clone(),
        }
    }
}

impl From<TableColumn> for crate::TableColumn {
    fn from(c: TableColumn) -> Self {
        Self {
            schema: c.table_schema,
            table_name: c.table_name,
            name: c.column_name,
            default: c.column_default,
            is_nullable: c.is_nullable,
            column_type: c.column_type,
            comment: c.column_comment,
            ..Default::default()
        }
    }
}

impl From<&TableColumn> for crate::TableColumn {
    fn from(c: &TableColumn) -> Self {
        let ty = mysql_2_rust(&c.column_type.clone().unwrap_or_default().to_uppercase());
        Self {
            schema: c.table_schema.clone(),
            table_name: c.table_name.clone(),
            name: Some(column_keywords(c.column_name.clone().unwrap().as_str())),
            default: c.column_default.clone(),
            is_nullable: {
                let ft = ty.0.clone();
                if ft.contains("Time") {
                    Some("Yes".to_string())
                } else {
                    c.is_nullable.clone()
                }
            },
            column_type: c.column_type.clone(),
            comment: c.column_comment.clone(),
            field_type: Some(ty.0.clone()),
            multi_world: Some({
                c.column_name
                    .clone()
                    .unwrap()
                    .contains(|c| c == '_' || c == '-')
            }),
        }
    }
}

/// FIXME：Mysql 类型转换为Rust对应类型
pub fn mysql_2_rust(t: &str) -> (String, String) {
    match t {
        "DECIMAL" => ("rbatis::Decimal".to_string(), "Decimal".to_string()),
        "BIGINT UNSIGNED" => ("u64".to_string(), "Number".to_string()),
        "BIGINT" => ("i64".to_string(), "Number".to_string()),
        "INT UNSIGNED" | "MEDIUMINT UNSIGNED" => ("u32".to_string(), "Number".to_string()),
        "INT" | "MEDIUMINT" => ("i32".to_string(), "Number".to_string()),
        "SMALLINT" => ("i16".to_string(), "Number".to_string()),
        "SMALLINT UNSIGNED" => ("u16".to_string(), "Number".to_string()),
        "TINYINT UNSIGNED" => ("u8".to_string(), "Number".to_string()),
        "TINYINT" => ("i8".to_string(), "Number".to_string()),
        "FLOAT" | "DOUBLE" => ("f64".to_string(), "Decimal".to_string()),
        "BINARY" | "VARBINARY" | "CHAR" | "VARCHAR" | "TEXT" | "ENUM" => {
            ("String".to_string(), "String".to_string())
        }
        "BLOB" | "TINYBLOB" | "MEDIUMBLOB" | "LONGBLOB" | "TINYTEXT" | "MEDIUMTEXT"
        | "LONGTEXT" => ("Vec<u8>".to_string(), "Collection".to_string()),
        "BIT" | "BOOLEAN" => ("u8".to_string(), "Number".to_string()),
        "DATE" => ("rbatis::DateNative".to_string(), "Date".to_string()),
        "TIME" | "YEAR" => ("rbatis::DateTimeNative".to_string(), "Date".to_string()),
        "DATETIME" => ("rbatis::DateTimeNative".to_string(), "Date".to_string()),
        "TIMESTAMP" => ("rbatis::Timestamp".to_string(), "Date".to_string()),
        "JSON" => ("Json<serde_json::Value>".to_string(), "String".to_string()),
        _ => ("Vec<u8>".to_string(), "Collection".to_string()),
    }
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
        CREATE_TIME;
    "
)]
async fn tables(table_names: &str) -> Vec<Table> {}

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
        ORDINAL_POSITION;
    "
)]
async fn columns(table_names: &str) -> Vec<TableColumn> {}