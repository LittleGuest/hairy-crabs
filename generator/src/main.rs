//! 代码生成器
//!
//! 指定数据库和表名，生成对应的struct
//!

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rbatis;

use std::{
    collections::HashMap,
    fs::{self, OpenOptions},
    io::Write,
    time::Duration,
};

use anyhow::Result;
use clap::Parser;
use convert::*;
use heck::CamelCase;
use rbatis::{db::DBPoolOptions, rbatis::Rbatis};
use table::*;
use template::*;

mod convert;
mod table;
mod template;

lazy_static! {
    static ref RB: Rbatis = Rbatis::new();
    static ref KEYWORDS: Vec<&'static str> = {
        // FIXME
        vec![
            "as", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern", "false",
            "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
            "ref", "return", "Self", "self", "static", "struct", "super", "trait", "true", "type",
            "unsafe", "use", "where", "while", "abstract", "async", "await", "become", "box", "do",
            "use", "final", "macro", "override", "priv", "try", "typeof", "unsized", "virtual",
            "yield",
        ]
    };
}

/// 生成器配置
#[derive(Parser, Debug)]
#[clap(version = "0.1.0", author = "2190975784@qq.com")]
struct Generator {
    /// 数据库驱动
    #[clap(short, long, default_value = "mysql")]
    driver: String,
    /// 数据库账号
    #[clap(short, long, default_value = "root")]
    username: String,
    /// 数据库密码
    #[clap(short, long, default_value = "root")]
    password: String,
    /// 数据库地址
    #[clap(long, default_value = "localhost")]
    url: String,
    /// 数据库端口号
    #[clap(short('P'), long, default_value = "3306")]
    port: u16,
    /// 指定的数据库名称
    #[clap(short('D'), long)]
    database: String,
    /// 代码生成的路径
    #[clap(long, default_value = "tps/")]
    path: String,
    /// 指定要生成代码的表名，多个用英文逗号拼接，为空表示全部
    #[clap(short('t'), long, default_value = "")]
    table_names: String,
}

impl Generator {
    /// 获取连接地址
    fn driver_url(&self) -> String {
        format!(
            "{}://{}:{}@{}:{}/{}",
            self.driver, self.username, self.password, self.url, self.port, self.database
        )
    }
}

impl ToString for Generator {
    fn to_string(&self) -> String {
        format!(
            "driver_url: {}://{}:{}@{}:{}/{}\npath: {}\ntable_names: {}",
            self.driver,
            self.username,
            self.password,
            self.url,
            self.port,
            self.database,
            self.path,
            self.table_names
        )
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut gen = Generator::parse();
    if !gen.path.is_empty() && !gen.path.ends_with("/") {
        gen.path.push_str("/")
    }
    println!("{}", gen.to_string());

    let mut pool_options = DBPoolOptions::new();
    pool_options.connect_timeout = Duration::from_secs(30);
    RB.link_opt(gen.driver_url().as_str(), pool_options).await?;
    generator(gen.path.as_str(), gen.table_names.as_str()).await?;

    Ok(())
}

/// 生成器
///
/// ```text
/// path: 指定生成路径
/// table_names: 指定生成的表明，为空则生成全部
/// ```
async fn generator(path: &str, table_names: &str) -> Result<()> {
    println!("====== start ======");
    let tables = table::tables(&table_names).await?;
    if tables.is_empty() {
        return Ok(());
    }

    let tables_columns = tables_columns(&table_names).await?;
    if tables_columns.is_empty() {
        return Ok(());
    }

    // 将tables转换为map，K：表名，V：表信息
    let table_map: HashMap<String, Table> = tables
        .into_iter()
        .map(|t| (t.table_name.to_owned().unwrap(), t))
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
                        .filter(|table_column| Some(table_name.clone()) == table_column.table_name)
                        .collect::<Vec<_>>(),
                );
                table_column_map
            });

    // 创建生成目录
    fs::create_dir_all(path)?;

    // 创建模板引擎
    let mut tera = tera::Tera::default();
    table_map.iter().for_each(|(table_name, table)| {
        let column = table_column_map.get(&table_name);
        // 创建上下文
        let mut ctx = tera::Context::new();
        ctx.insert("struct_name", &table_name.to_camel_case());
        ctx.insert("table", &table);
        let mut has_columns = false;
        if let Some(columns) = column {
            has_columns = !columns.is_empty();

            let cols = columns.iter().fold(Vec::new(), |mut cols, column| {
                let mysql_2_rust =
                    mysql_2_rust(&column.data_type.clone().unwrap_or_default().to_uppercase());
                cols.push(TableColumn {
                    table_schema: column.table_schema.clone(),
                    table_name: column.table_name.clone(),
                    column_name: {
                        // 列名是否为Rust关键字，若为关键字，则需要在其前加 r#
                        let column_name = column.column_name.clone().unwrap();
                        if KEYWORDS.contains(&column_name.as_str()) {
                            Some(format!("r#{}", column_name))
                        } else {
                            Some(column_name)
                        }
                    },
                    ordinal_position: column.ordinal_position,
                    column_default: column.column_default.clone(),
                    is_nullable: {
                        let ft = mysql_2_rust.0.clone();
                        if ft.contains("Time") {
                            Some("Yes".to_string())
                        } else {
                            column.is_nullable.clone()
                        }
                    },
                    data_type: column.data_type.clone(),
                    character_maximum_length: column.character_maximum_length,
                    column_type: column.column_type.clone(),
                    column_key: column.column_key.clone(),
                    column_comment: column.column_comment.clone(),
                    field_type: Some(mysql_2_rust.0.clone()),
                    multi_world: Some({
                        column
                            .column_name
                            .clone()
                            .unwrap()
                            .contains(|c| c == '_' || c == '-')
                    }),
                    r#type: { Some(mysql_2_rust.1) },
                });
                cols
            });
            ctx.insert("columns", &cols);
        }
        ctx.insert("has_columns", &has_columns);

        // 渲染模板
        let render_string = tera.render_str(MODEL_TEMPLATE, &ctx).expect("渲染模板错误");
        // 创建文件
        let file_name = format!("{}{}.rs", path, &table_name);
        let mut tf = fs::File::create(&file_name).expect("创建文件失败");
        tf.write_all(render_string.as_bytes())
            .expect("写入数据错误");

        println!("the {} has been generated", &file_name);
    });

    let mut ctx = tera::Context::new();
    ctx.insert("table_names", &table_map);
    let render_string = tera.render_str(MOD_TEMPLATE, &ctx)?;

    // 创建 mod.rs 文件
    let mod_file_name = format!("{}mod.rs", path);

    let mut tf = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&mod_file_name)?;
    tf.write_all(render_string.as_bytes())?;

    println!("the {} has been generated", &mod_file_name);
    println!("====== over ======");
    Ok(())
}
