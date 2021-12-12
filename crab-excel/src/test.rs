use std::process::Output;

use calamine::{open_workbook, DataType, Reader, Xls, Xlsx};
use xlsxwriter::{FormatAlignment, FormatColor, FormatUnderline, Workbook};

#[test]
fn test_read() {
    let mut workbook: Xlsx<_> = open_workbook("用户数据.xlsx").expect("Cannot open file");

    let sheets = workbook.sheet_names().to_owned();
    for s in sheets {
        let w = workbook.worksheet_range(&s).unwrap().unwrap();
        println!("{:?}", w);
    }
}

#[test]
fn test_write() {
    let workbook = Workbook::new("simple_style.xlsx");
    let mut format1 = workbook.add_format().set_font_color(FormatColor::Red);

    let mut format2 = workbook
        .add_format()
        .set_font_color(FormatColor::Blue)
        .set_underline(FormatUnderline::Single);

    let mut format3 = workbook
        .add_format()
        .set_font_color(FormatColor::Green)
        .set_align(FormatAlignment::CenterAcross)
        .set_align(FormatAlignment::VerticalCenter);

    let mut sheet1 = workbook.add_worksheet(None).unwrap();
    sheet1
        .write_string(0, 0, "Red text", Some(&format1))
        .unwrap();
    sheet1.write_number(0, 1, 20., None).unwrap();
    sheet1.write_formula_num(1, 0, "=10+B1", None, 30.).unwrap();
    sheet1
        .write_url(
            1,
            1,
            "https://github.com/informationsea/xlsxwriter-rs",
            Some(&format2),
        )
        .unwrap();
    sheet1
        .merge_range(2, 0, 3, 2, "Hello, world", Some(&format3))
        .unwrap();

    sheet1.set_selection(1, 0, 1, 2);
    sheet1.set_tab_color(FormatColor::Cyan);
    workbook.close().unwrap();
}

use simple_excel_writer::*;

#[test]
fn test_write_simple() {
    let mut wb = simple_excel_writer::Workbook::create("simple.xlsx");
    let mut sheet = wb.create_sheet("SheetName");

    // set column width
    sheet.add_column(Column { width: 30.0 });
    sheet.add_column(Column { width: 30.0 });
    sheet.add_column(Column { width: 80.0 });
    sheet.add_column(Column { width: 60.0 });

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

    let mut sheet = wb.create_sheet("Sheet2");
    wb.write_sheet(&mut sheet, |sheet_writer| {
        let sw = sheet_writer;
        sw.append_row(row!["Name", "Title", "Success", "Remark"])?;
        sw.append_row(row!["Amy", "Manager", true])
    })
    .expect("write excel error!");

    let excel_data = wb.close().expect("close excel error!");
}
