use uuid::Uuid;

use crate::application::ports::outbound::idgenerator::IdGenerator;

pub struct Uuidv4Generator;

impl Uuidv4Generator {
    pub fn new() -> Self {
        Self
    }
}

impl IdGenerator for Uuidv4Generator {
    fn create(&self) -> Result<String, String> {
        Ok(Uuid::new_v4().to_string())
    }
}