use async_trait::async_trait;
use rbatis::crud::CRUD;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{column_keywords, multi_world, Database, RB};

pub struct Sqlite;

#[async_trait]
impl Database for Sqlite {
    async fn tables(&self, table_names: &[&str]) -> Vec<crate::Table> {
        let ts = tables(table_names).await.unwrap();
        ts.iter().map(|t| t.into()).collect::<Vec<_>>()
    }
    async fn columns(&self, table_names: &[&str]) -> Vec<crate::TableColumn> {
        let mut cols = vec![];
        for table_name in table_names.iter() {
            let columns = columns(table_name).await.unwrap();
            println!("== {:?}", columns);
            let mut columns = columns
                .iter()
                .map(|c| c.into())
                .collect::<Vec<crate::TableColumn>>()
                .iter_mut()
                .map(|c| {
                    c.table_name = Some(table_name.to_string());
                    c.to_owned()
                })
                .collect::<Vec<_>>();
            cols.append(&mut columns);
        }
        cols
    }
}

/// 表信息来自 sqlite_master
#[crud_table]
#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all(deserialize = "SCREAMING_SNAKE_CASE"))]
pub struct Table {
    /// 项目的类型：table，index，view，trigger
    r#type: Option<String>,
    /// 项目的名称
    name: Option<String>,
    /// 所从属的表名，如索引所在的表名
    tbl_name: Option<String>,
    /// 项目在数据库页中存储的编号
    rootpage: Option<i64>,
    /// SQL语句
    sql: Option<String>,
}

/// 表列信息
#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all(deserialize = "SCREAMING_SNAKE_CASE"))]
pub struct TableColumn {
    /// 列ID
    cid: Option<u64>,
    /// 列名
    name: Option<String>,
    /// 类型：如：varchar(50)  int
    r#type: Option<String>,
    /// 是否为空：1-不为空，0-为空
    notnull: Option<u8>,
    dflt_value: Option<String>,
    /// 是否为主键：1-主键，0-非主键
    pk: Option<u8>,
}

impl From<Table> for crate::Table {
    fn from(t: Table) -> Self {
        Self {
            name: t.name,
            ..Default::default()
        }
    }
}

impl From<&Table> for crate::Table {
    fn from(t: &Table) -> Self {
        Self {
            name: t.name.clone(),
            ..Default::default()
        }
    }
}

impl From<TableColumn> for crate::TableColumn {
    fn from(col: TableColumn) -> Self {
        let ty = sqlite_type(col.r#type.clone().unwrap().as_str());
        Self {
            name: Some(column_keywords(col.name.clone().unwrap().as_str())),
            default: col.dflt_value,
            is_nullable: {
                if let Some(is_null) = col.notnull {
                    if is_null == 1 {
                        Some("NotNull".to_string())
                    } else {
                        Some("Null".to_string())
                    }
                } else {
                    None
                }
            },
            column_type: col.r#type.clone(),
            field_type: Some(sqlite_2_rust(ty.0.as_str())),
            multi_world: Some(multi_world(col.name.clone().unwrap().as_str())),
            ..Default::default()
        }
    }
}

impl From<&TableColumn> for crate::TableColumn {
    fn from(col: &TableColumn) -> Self {
        let ty = sqlite_type(col.r#type.clone().unwrap().as_str());
        Self {
            name: Some(column_keywords(col.name.clone().unwrap().as_str())),
            default: col.dflt_value.clone(),
            is_nullable: {
                if let Some(is_null) = col.notnull {
                    if is_null == 1 {
                        Some("NotNull".to_string())
                    } else {
                        Some("Null".to_string())
                    }
                } else {
                    None
                }
            },
            column_type: col.r#type.clone(),
            field_type: Some(sqlite_2_rust(ty.0.as_str())),
            multi_world: Some(multi_world(col.name.clone().clone().unwrap().as_str())),
            ..Default::default()
        }
    }
}

/// FIXME:Sqlite类型转换为Rust类型
fn sqlite_2_rust(t: &str) -> String {
    match t.to_lowercase().as_str() {
        "text" | "char" => "String".to_string(),
        "int" => "i64".to_string(),
        "date" | "datetime" => "u64".to_string(),
        _ => "String".to_string(),
    }
}

/// 根据sqlite字段类型截取类型和长度
/// date、datetime、int没有长度
/// varchar有长度
fn sqlite_type(t: &str) -> (String, Option<u16>) {
    let rg = Regex::new("^(.*)\\((\\d+)\\)$").unwrap();
    if let Some(caps) = rg.captures(t) {
        (
            caps.get(1).map_or("".to_string(), |tt| tt.as_str().into()),
            caps.get(2)
                .map_or(Some(0), |l| Some(l.as_str().parse::<u16>().unwrap_or(0))),
        )
    } else {
        (t.to_string(), None)
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
    SELECT type, name, tbl_name, rootpage, sql
    FROM sqlite_master
    WHERE type = 'table'
        AND (
        1 = 2
    trim ',': for item in table_names:
        OR name = '#{item}'
        )
    ORDER by rootpage;
    "
)]
async fn tables(table_names: &[&str]) -> Vec<Table> {}

/// 获取指定表的列信息
///
/// ```text
/// table_name：单个表名
/// ```
///
#[py_sql(
    RB,
    "
    pragma table_info('#{table_names}');
    "
)]
async fn columns(table_name: &str) -> Vec<TableColumn> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqlite_type() {
        assert_eq!(
            ("varchar".to_string(), Some(64)),
            sqlite_type("varchar(64)")
        );
        assert_eq!(("char".to_string(), Some(1)), sqlite_type("char(1)"));
        assert_eq!(("date".to_string(), None), sqlite_type("date"));
        assert_eq!(("datetime".to_string(), None), sqlite_type("datetime"));
        assert_eq!(("int".to_string(), None), sqlite_type("int"));
    }
}
