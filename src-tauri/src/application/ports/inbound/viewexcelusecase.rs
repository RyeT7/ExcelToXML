use crate::domain::datastructures::table::Table;

pub trait ViewExcelUseCase: Send + Sync {
    fn view_excel(&self, session_id: &str) -> Result<Table, String>;
}