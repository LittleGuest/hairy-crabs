/// FIXME：简单转换
pub(crate) fn mysql_2_rust(t: &str) -> String {
    match t {
        "DECIMAL" => "rbatis::Decimal".to_string(),
        "BIGINT UNSIGNED" => "u64".to_string(),
        "BIGINT" => "i64".to_string(),
        "INT UNSIGNED" | "MEDIUMINT UNSIGNED" => "u32".to_string(),
        "INT" | "MEDIUMINT" => "i32".to_string(),
        "SMALLINT" => "i16".to_string(),
        "SMALLINT UNSIGNED" => "u16".to_string(),
        "TINYINT UNSIGNED" => "u8".to_string(),
        "TINYINT" => "i8".to_string(),
        "FLOAT" | "DOUBLE" => "f64".to_string(),
        "BINARY" | "VARBINARY" | "CHAR" | "VARCHAR" | "TEXT" | "ENUM" => "String".to_string(),
        "BLOB" | "TINYBLOB" | "MEDIUMBLOB" | "LONGBLOB" | "TINYTEXT" | "MEDIUMTEXT"
        | "LONGTEXT" => "Vec<u8>".to_string(),
        "BIT" | "BOOLEAN" => "u8".to_string(),
        "DATE" => "rbatis::DateNative".to_string(),
        "TIME" | "YEAR" => "rbatis::DateTimeNative".to_string(),
        "DATETIME" => "rbatis::DateTimeNative".to_string(),
        "TIMESTAMP" => "rbatis::Timestamp".to_string(),
        "JSON" => "Json<serde_json::Value>".to_string(),
        _ => "Vec<u8>".to_string(),
    }
}
