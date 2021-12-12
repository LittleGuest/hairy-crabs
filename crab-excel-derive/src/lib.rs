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

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Result};

use crate::excel_col::ExcelCol;

#[proc_macro_derive(ExcelWriter, attributes(excel_col))]
pub fn excel_macro_derive(input: TokenStream) -> TokenStream {
    // 解析为语法树
    let ast = parse_macro_input!(input as DeriveInput);

    // for attr in &ast.attrs {
    //     println!("ast.attrs {:?}", attr.path.get_ident());
    // }

    resolve(&ast);

    impl_excel_writer_macro(&ast)
}

fn resolve(ast: &DeriveInput) {
    match &ast.data {
        syn::Data::Struct(ds) => match &ds.fields {
            syn::Fields::Named(fsn) => {
                let mut excel_cols = vec![];
                for field in &fsn.named {
                    println!("{}", ["="; 20].join(""));
                    // println!("field.ident {:?}", field.ident);

                    for field_attr in &field.attrs {
                        // println!("field_attr.path {:?}", field_attr.path.get_ident());
                        match field_attr.parse_args::<ExcelCol>() {
                            Ok(ec) => {
                                // TODO excel_col中的属性解析成功
                                println!("parse success");
                                excel_cols.push(ec);
                            }
                            Err(e) => println!("{:?}", e.to_string()),
                        }
                    }
                }
            }
            syn::Fields::Unnamed(_fsu) => todo!(),
            syn::Fields::Unit => todo!(),
        },
        syn::Data::Enum(_de) => todo!(),
        syn::Data::Union(_) => (),
    }
}

fn impl_excel_writer_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let expand = quote! {
        use simple_excel_writer::*;

        impl ExcelWriter for #name {
            fn simple_write(data: &[Self]) -> Result<Vec<u8>, err::ExcelError>{
                let mut wb = Workbook::create("simple.xlsx");
                let mut sheet = wb.create_sheet("SheetName");

                // 设置列的宽度
                sheet.add_column(Column { width: 30.0 });
                sheet.add_column(Column { width: 30.0 });

                wb.write_sheet(&mut sheet, |sheet_writer| {
                    let sw = sheet_writer;
                    sw.append_row(row!["Name", "Title", "Success", "XML Remark"])?;
                    sw.append_row(row![
                        "Amy",
                        (),
                        true,
                        "<xml><tag>\"Hello\" & 'World'</tag></xml>"
                    ])?;
                    sw.append_blank_rows(2);
                    sw.append_row(row!["Tony", blank!(2), "retired"])
                })
                .expect("write excel error!");


                let excel_data = wb.close().expect("close excel error!").unwrap_or_else(||vec![]);
                Ok(excel_data)
            }
        }
    };

    expand.into()
}
