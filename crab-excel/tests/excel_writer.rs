use crab_excel::writer::ExcelWriter;
use crab_excel_derive::ExcelWriter;

#[derive(Debug, Default, ExcelWriter)]
struct TestExcel {
    #[excel_col(name = "用户ID", sort = 1)]
    id: u32,
    #[excel_col(name = "用户昵称")]
    name: String,
    #[excel_col(sort = 1)]
    roles: Vec<u32>,
    #[excel_col(name = "是否删除", sort = 2)]
    del: u8,
    #[excel_col(sort = 1, name = "日期")]
    date: u128,
}

#[test]
fn simple_excel_writer() {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    let data = [
        TestExcel {
            id: 1,
            name: "用户昵称".to_string(),
            roles: vec![88; 8],
            del: 0,
            date: now.as_nanos(),
        },
        TestExcel {
            id: 1,
            name: "用户昵称".to_string(),
            roles: vec![88; 8],
            del: 0,
            date: now.as_nanos(),
        },
        TestExcel {
            id: 1,
            name: "用户昵称".to_string(),
            roles: vec![88; 8],
            del: 0,
            date: now.as_nanos(),
        },
    ];

    let _ = TestExcel::simple_write(&data).unwrap();
}
