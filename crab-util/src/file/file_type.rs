//! 文件类型工具类

// /// 获取文件类型
// async fn file_type(mut file: &mut poem::web::Multipart) -> &str {
//     // let field = file.next_field().await;
//     // let file_name = field.file_name().unwrap_or_else(|| "");
//     // file_name
//     ""
// }

/// 通过文件名获取文件类型
async fn file_type_by_name(file_name: &str) -> &str {
    match file_name.trim().rfind('.') {
        Some(i) => file_name.split_at(i).1,
        None => "",
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_get_file_type_by_name() {
        // assert_eq!("", block_on(file_type_by_name("")));
        // assert_eq!("", block_on(file_type_by_name(" ")));
        // assert_eq!("", block_on(file_type_by_name(".")));
        // assert_eq!("", block_on(file_type_by_name(" . ")));
        // assert_eq!("", block_on(file_type_by_name("file")));
        // assert_eq!("bin", block_on(file_type_by_name("file.bin")));
        // assert_eq!("bin", block_on(file_type_by_name(".bin")));
    }
}
