pub trait CreateSessionUseCase: Send + Sync {
    fn create_session(&self) -> Result<String, String>;
}