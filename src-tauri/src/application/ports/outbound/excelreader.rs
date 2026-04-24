use crate::domain::datastructures::table::Table;

pub trait ExcelReader: Send + Sync {
    fn load_excel(&self, path: &str) -> Result<Table, String>;
}