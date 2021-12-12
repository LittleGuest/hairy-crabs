#[derive(Debug)]
pub enum ExcelError {
    Io,
}

impl std::fmt::Display for ExcelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExcelError::Io => writeln!(f, "io error"),
        }
    }
}
