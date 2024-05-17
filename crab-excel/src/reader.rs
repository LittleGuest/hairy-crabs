//! 读 Excel

pub trait ExcelReader {
    fn read<T>() -> Vec<T>;
}
