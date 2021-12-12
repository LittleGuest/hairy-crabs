//! Excel 导入导出

#![allow(unused)]

pub mod err;
pub mod reader;
#[cfg(test)]
mod test;
pub mod writer;

use std::vec;

use crab_excel_derive::ExcelWriter;
use writer::ExcelWriter;

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

#[derive(Debug, Default, ExcelWriter)]
struct TestExcel {
    #[excel_col(name = "1", order = 1)]
    id: u32,
    #[excel_col(name = "TestExcel")]
    name: String,
    #[excel_col(order = 1)]
    roles: Vec<u32>,
    #[excel_col(name = "Rust", order = 2)]
    del: u8,
    #[excel_col(order = 1, name = "Rust")]
    date: u128,
}

#[test]
fn test_excel() {
    let data = vec![
        TestExcel {
            id: 1001,
            name: "Rust".to_string(),
            roles: vec![12, 3, 4],
            del: 0,
            date: 1639233721000,
        },
        TestExcel::default(),
        TestExcel {
            id: 1003,
            name: "Crab".to_string(),
            roles: vec![4],
            del: 0,
            date: 1639233721000,
        },
        TestExcel::default(),
        TestExcel {
            id: 1005,
            name: "Derive".to_string(),
            roles: vec![],
            del: 0,
            date: 1639233721000,
        },
    ];
    TestExcel::simple_write(&data);
}
