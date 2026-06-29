pub trait ConvertUseCase: Send + Sync {
    fn convert(&mut self, session_id: &str, tin: &str) -> Result<(), String>;
}
