pub trait UploadExcelUseCase: Send + Sync {
    fn load_excel(
        &self,
        session_id: &str,
        path: &str
    ) -> Result<(), String>;
}