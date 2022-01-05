//! Excel 相关宏

/*
设计

#[derive(Excel)]
struct User{
    #[excel_col(name = "用户ID", order = 1,width = 14, type = Type::r#String)]
    id: u32,
    name: String,
    role_name: String,
    #[excel_col(name = "创建时间", order = 2, date_format = "yyyy-MM-dd HH:mm::ss")]
    create_time: u32,
}

Excel 自定义派生宏
定义为 trait，只能用于struct，只要实现了该trait，就具备到导入和导出的能力

trait Excel{
    type output = Self
    fn write(&self,data: &[output])

}

excel_col 属性设计
name: 列名称
order: 序号，指定表头的顺序，从左到右，order的值越大
width: 单元格的宽度
target_type: 导出的类型:
        String：文本格式，默认值
        Num： 数字格式，
        Fn：函数格式，
        Img：图片
        Date：日期格式
date_format: 日期格式化，如 yyyy-MM-dd HH:mm:ss

*/

#![allow(unused)]

mod excel_col;

use darling::{FromDeriveInput, ToTokens};
use excel_col::ExcelWriterMacro;
use proc_macro::{Literal, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident, LitStr, Result};

use crate::excel_col::ExcelCol;

#[proc_macro_derive(ExcelWriter, attributes(excel_col))]
pub fn excel_writer_macro_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let ewm = ExcelWriterMacro::from_derive_input(&ast).unwrap();
    if ewm.data.is_struct() {
        impl_excel_writer_macro(ewm)
    } else {
        token(ast)
    }
}

fn token(ast: DeriveInput) -> TokenStream {
    let expand = quote! {
        #ast
    };
    expand.into()
}

fn impl_excel_writer_macro(ewm: ExcelWriterMacro) -> TokenStream {
    let struct_name_token = &ewm.ident;

    let mut field_names = vec![];
    &ewm.data.map_struct_fields(|el| {
        let field_name = el.ident.unwrap().to_string();
        field_names.push(field_name.clone());
        let mut col_name = el.name.unwrap_or_else(|| "".to_string());

        println!("field_name = {}", field_name);
        println!("col_name = {}", col_name);
    });

    println!("{:?}", field_names);
    let mut fnq = quote! {};

    for ins in field_names.iter() {
        fnq = quote! {
            #fnq
            row.add_cell(#ins);
        };
    }

    fnq = quote! {
        {
            let mut row = Row::new();
        #fnq
        row
        }
    };

    let mut rows = quote! {};
    for fd in field_names.iter() {
        rows = quote! {
            #rows

            sw.append_row({
                let mut row = Row::new();

                row

            });
        };

        // sw.append_row(row![
        //     "Amy",
        //     (),
        //     true,
        //     "<xml><tag>\"Hello\" & 'World'</tag></xml>"
        // ])?;
        // sw.append_blank_rows(2);
        // sw.append_row(row!["Tony", blank!(2), "retired"])
    }

    let expand = quote! {
        use simple_excel_writer::*;

        impl ExcelWriter for #struct_name_token {
            type Input = Self;
            fn simple_write(data: &[Self::Input]) -> Result<Option<Vec<u8>>, crab_excel::ExcelError>{
                let mut wb = Workbook::create("测试数据.xlsx");
                let mut sheet = wb.create_sheet("测试数据");

                // sheet.add_column(Column { width: 30.0 });
                // sheet.add_column(Column { width: 30.0 });

                // 设置列的宽度
                sheet.add_column(Column { width: 30.0 });
                sheet.add_column(Column { width: 30.0 });

                wb.write_sheet(&mut sheet, |sheet_writer| {
                    let sw = sheet_writer;
                    // sw.append_row(row![
                    //     "Amy",
                    //     (),
                    //     true,
                    //     "<xml><tag>\"Hello\" & 'World'</tag></xml>"
                    // ])?;
                    // sw.append_blank_rows(2);
                    // sw.append_row(row!["Tony", blank!(2), "retired"])

                    sw.append_row(#fnq)?;

                    for dt in data.iter(){
                        let mut row = Row::new();
                        for fd in 0..4{
                            row.add_cell(dt.name.clone());
                        }
                        sw.append_row(row);
                    }
                    Ok(())
                })
                .expect("write excel error!");

                wb.close().map_err(|e|crab_excel::ExcelError::E("close excel error!".to_string()))
            }
        }
    };

    println!("{:?}", expand.to_string());
    expand.into()
}
