use std::{
    collections::HashMap,
    error::Error,
    fmt::Display,
    fs::{self, OpenOptions},
    io::Write,
    str::FromStr,
    time::Duration,
};

use async_trait::async_trait;
use clap::{ArgEnum, Parser};
use heck::ToUpperCamelCase;
use rbatis::{db::DBPoolOptions, rbatis::Rbatis};
use serde::{Deserialize, Serialize};

use crate::template::{MODEL_TEMPLATE, MOD_TEMPLATE};

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rbatis;

pub mod mysql;
pub mod sqlite;
pub mod template;

lazy_static! {
    pub static ref RB: Rbatis = Rbatis::new();
    pub static ref KEYWORDS: Vec<&'static str> = {
        // FIXME Rust1.60 关键字
        vec![
            "as", "async", "await","break", "const", "continue", "crate", "dyn", "else", "enum", "extern", "false",
            "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
            "ref", "return", "Self", "self", "static", "struct", "super", "trait", "true", "type","union",
            "unsafe", "use", "where", "while", "abstract",  "become", "box", "do",
             "final", "macro", "override", "priv", "try", "typeof", "unsized", "virtual",
            "yield",
        ]
    };
}

pub const MYSQL: &str = "mysql";
pub const SQLITE: &str = "sqlite";

/// 驱动类型
#[derive(Debug, Clone, Copy, ArgEnum)]
pub enum Driver {
    Sqlite,
    Mysql,
}

impl FromStr for Driver {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case(MYSQL) {
            Ok(Driver::Mysql)
        } else if s.eq_ignore_ascii_case(SQLITE) {
            Ok(Driver::Sqlite)
        } else {
            Err("不支持的驱动类型")
        }
    }
}

impl Display for Driver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Driver::Sqlite => write!(f, "{}", SQLITE),
            Driver::Mysql => write!(f, "{}", MYSQL),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all(deserialize = "SCREAMING_SNAKE_CASE"))]
pub struct Table {
    pub schema: Option<String>,
    pub name: Option<String>,
    pub comment: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all(deserialize = "SCREAMING_SNAKE_CASE"))]
pub struct TableColumn {
    pub schema: Option<String>,
    pub table_name: Option<String>,
    pub name: Option<String>,
    pub default: Option<String>,
    pub max_length: Option<i64>,
    pub is_nullable: Option<String>,
    pub column_type: Option<String>,
    pub comment: Option<String>,

    // 对应 Rust 类型
    pub field_type: Option<String>,
    pub multi_world: Option<bool>,
}

#[async_trait]
pub trait Database {
    /// 获取指定表信息
    async fn tables(&self, table_names: &[&str]) -> Vec<Table>;
    /// 获取指定表的字段
    async fn columns(&self, table_names: &[&str]) -> Vec<TableColumn>;
}

/// 生成器配置
#[derive(Parser, Debug)]
#[clap(version = "0.1.0", author = "2190975784@qq.com")]
pub struct Generator {
    /// 数据库驱动
    #[clap(short, default_value = "mysql")]
    pub driver: Driver,
    /// 数据库账号
    #[clap(short, default_value = "root")]
    pub username: String,
    /// 数据库密码
    #[clap(short, default_value = "root")]
    pub password: String,
    /// 数据库地址
    #[clap(short, default_value = "localhost")]
    pub host: String,
    /// 数据库端口号
    #[clap(short('P'), default_value_t = 3306)]
    pub port: u16,
    /// 指定的数据库名称
    #[clap(short('D'))]
    pub database: String,
    /// 代码生成的路径
    #[clap(default_value = "model/")]
    pub path: String,
    /// 指定要生成代码的表名，多个用英文逗号拼接，为空表示全部
    #[clap(short('t'), long, default_value = "")]
    pub table_names: String,
}

impl Display for Generator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"
            driver_url: {}://{}:{}@{}:{}/{}
            path: {}
            table_names: {}
           "#,
            self.driver,
            self.username,
            self.password,
            self.host,
            self.port,
            self.database,
            self.path,
            self.table_names
        )
    }
}

impl Generator {
    pub fn driver_url(&self) -> String {
        match self.driver {
            Driver::Sqlite => format!("sqlite://{}", self.database),
            Driver::Mysql => format!(
                "mysql://{}:{}@{}:{}/{}",
                self.username, self.password, self.host, self.port, self.database
            ),
        }
    }

    /// 初始化数据库连接
    pub async fn init(&self) -> Result<(), Box<dyn Error>> {
        let mut pool_options = DBPoolOptions::new();
        pool_options.connect_timeout = Duration::from_secs(30);
        RB.link_opt(self.driver_url().as_str(), pool_options)
            .await?;
        Ok(())
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        self.init().await?;
        self.deal_path();
        self.generator().await?;
        Ok(())
    }

    ///  处理路径，当路径不以 / 结尾时，自动添加 /
    fn deal_path(&mut self) {
        if !self.path.is_empty() && !self.path.ends_with('/') {
            self.path.push('/')
        }
    }

    /// 生成器
    ///
    /// ```text
    /// path: 指定生成路径
    /// table_names: 指定生成的表明，为空则生成全部
    /// ```
    pub async fn generator(&self) -> Result<(), Box<dyn Error>> {
        println!("{self}");
        println!("====== start ======");

        // TODO 什么是 trait object?
        let tobj: &dyn Database = {
            match self.driver {
                Driver::Sqlite => &sqlite::Sqlite,
                Driver::Mysql => &mysql::Mysql,
            }
        };

        let table_names = self.table_names.split(',').collect::<Vec<_>>();

        let tables = tobj.tables(&table_names).await;
        if tables.is_empty() {
            return Ok(());
        }

        let tables_columns = tobj.columns(&table_names).await;
        if tables_columns.is_empty() {
            return Ok(());
        }

        // 将tables转换为map，K：表名，V：表信息
        let table_map: HashMap<String, Table> = tables
            .into_iter()
            .map(|t| (t.name.to_owned().unwrap(), t))
            .collect();

        // 组装表信息和表列信息，K：表名，V：表列信息
        // TODO：有没有办法直接将Vec分组，类似Java的Collectors.groupby
        let table_column_map =
            table_map
                .keys()
                .fold(HashMap::new(), |mut table_column_map, table_name| {
                    table_column_map.insert(
                        table_name,
                        tables_columns
                            .iter()
                            .filter(|table_column| {
                                Some(table_name.clone()) == table_column.table_name
                            })
                            .collect::<Vec<_>>(),
                    );
                    table_column_map
                });

        // 创建生成目录
        fs::create_dir_all(&self.path)?;

        // 创建模板引擎
        let mut tera = tera::Tera::default();
        table_map.iter().for_each(|(table_name, table)| {
            let column = table_column_map.get(&table_name);
            // 创建上下文
            let mut ctx = tera::Context::new();
            ctx.insert("struct_name", &table_name.to_upper_camel_case());
            ctx.insert("table", &table);
            let mut has_columns = false;
            if let Some(columns) = column {
                has_columns = !columns.is_empty();
                ctx.insert("columns", &columns);
            }
            ctx.insert("has_columns", &has_columns);

            // 渲染模板
            let render_string = tera.render_str(MODEL_TEMPLATE, &ctx).expect("渲染模板错误");
            // 创建文件
            let file_name = format!("{}{}.rs", self.path, &table_name);
            let mut tf = fs::File::create(&file_name).expect("创建文件失败");
            tf.write_all(render_string.as_bytes())
                .expect("写入数据错误");

            println!("the {} has been generated", &file_name);
        });

        let mut ctx = tera::Context::new();
        ctx.insert("table_names", &table_map);
        let render_string = tera.render_str(MOD_TEMPLATE, &ctx)?;

        // 创建 mod.rs 文件
        let mod_file_name = format!("{}mod.rs", self.path);

        let mut tf = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&mod_file_name)?;
        tf.write_all(render_string.as_bytes())?;

        println!("the {} has been generated", &mod_file_name);
        println!("====== over ======");
        Ok(())
    }
}

/// 判断字段名称是否是由多个单词组成
fn multi_world(name: &str) -> bool {
    name.contains(|c| c == '_' || c == '-')
}

/// 列名是否为Rust关键字，若为关键字，则需要在其前加 r#
fn column_keywords(name: &str) -> String {
    if KEYWORDS.contains(&name) {
        format!("r#{}", name)
    } else {
        name.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen_mysql() {
        let mut gen = Generator {
            driver: Driver::Mysql,
            username: "root".to_string(),
            password: "root".to_string(),
            host: "127.0.0.1".to_string(),
            port: 3306,
            database: "aidex".to_string(),
            path: "./tps".to_string(),
            table_names: "sys_user".to_string(),
        };
        tokio_test::block_on(gen.run()).unwrap();
    }

    #[test]
    fn gen_sqlite() {
        let mut gen = Generator {
            driver: Driver::Sqlite,
            username: "root".to_string(),
            password: "root".to_string(),
            host: "127.0.0.1".to_string(),
            port: 3306,
            database: "../hairy-crabs.sqlite".to_string(),
            path: "./tps".to_string(),
            table_names: "sys_user".to_string(),
        };
        tokio_test::block_on(gen.run()).unwrap();
    }
}
