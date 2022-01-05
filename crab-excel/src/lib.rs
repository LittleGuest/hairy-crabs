//! Excel 导入导出

#![allow(unused)]

pub mod reader;
pub mod writer;

use std::vec;

use crab_excel_derive::ExcelWriter;
use thiserror::Error;
use writer::ExcelWriter;

#[derive(Debug, Error)]
pub enum ExcelError {
    #[error("{0}")]
    E(String),
}

// /// 导出的类型Type
// pub enum Type {
//     /// 文本格式，默认值
//     String,
//     ///  数字格式
//     Num,
//     /// 函数格式
//     Fn,
//     /// 图片
//     Img,
//     /// 日期格式
//     Date,
// }

