//! 代码生成器
//!
//! 指定数据库和表名，生成对应的struct
//!

use std::error::Error;

use clap::StructOpt;
use generator::Generator;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut gen = Generator::parse();
    gen.run().await?;
    Ok(())
}
