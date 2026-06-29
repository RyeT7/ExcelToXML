pub trait GetXmlUseCase: Send + Sync {
    fn get_xml(&self, session_id: &str) -> Result<String, String>;
}
