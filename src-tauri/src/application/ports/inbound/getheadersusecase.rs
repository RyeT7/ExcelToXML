pub trait GetHeadersUseCase: Send + Sync {
    fn get_headers(&self, session_id: &str) -> Result<Vec<String>, String>;
}