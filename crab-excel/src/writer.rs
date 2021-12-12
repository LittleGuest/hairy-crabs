//! 写Excel

use std::{
    path::{Path, PathBuf},
    str::FromStr,
    time::SystemTime,
};

use crate::err;

/// Excel导出参数
#[derive(Debug)]
pub struct ExportParam {
    /// 文件名
    file_name: &'static str,
    /// 文件路径
    path: PathBuf,
}

impl ExportParam {
    fn new() -> Self {
        let file_name = Self::file_name();
        Self {
            file_name: file_name,
            path: {
                let pb = PathBuf::new();
                pb.with_file_name(file_name);
                pb
            },
        }
    }

    fn file_name() -> &'static str {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(d) => {
                let fin = format!("{}.xlsx", d.as_millis());
                // TODO
                "export.xlsx"
            }
            Err(_) => "export.xlsx",
        }
    }
}

pub trait ExcelWriter: Sized {
    fn simple_write(data: &[Self]) -> Result<Vec<u8>, err::ExcelError>;
}
