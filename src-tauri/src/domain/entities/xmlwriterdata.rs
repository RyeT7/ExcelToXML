pub struct XMLWriterData<'a> {
    xml: String,
    tags: Vec<&'a str>,
    last_padding: usize,
}

impl<'a> XMLWriterData<'a> {
    pub fn new() -> XMLWriterData<'a> {
        XMLWriterData {
            xml             : String::new(),
            last_padding    : 0,
            tags            : Vec::new(),
        }
    }

    pub fn get_xml(&self) -> &str {
        &self.xml
    }

    pub fn get_tags(&self) -> &Vec<&str> {
        &self.tags
    }

    pub fn get_last_padding(&self) -> &usize {
        &self.last_padding
    }
}