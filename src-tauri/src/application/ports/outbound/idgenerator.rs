pub trait IdGenerator: Send + Sync {
    fn create(&self) -> Result<String, String>;
}