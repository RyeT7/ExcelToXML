pub struct ExcelReaderResponse {
    pub errors: Vec<String>,
}

pub trait ExcelReaderResponseTrait {
    fn new() -> ExcelReaderResponse;
    fn add_error(&mut self, error: String);
}

impl ExcelReaderResponseTrait for ExcelReaderResponse {
    fn new() -> ExcelReaderResponse {
        ExcelReaderResponse {
            errors: Vec::new(),
        }
    }

    fn add_error(&mut self, error: String) {
        self.errors.push(error.to_string());
    }
}