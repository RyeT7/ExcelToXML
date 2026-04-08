use crate::xml::attributes::XMLAttributes;

struct XMLWriter<'a> {
    xml: String,
    tags: Vec<&'a str>,
    last_padding: usize,
}

pub trait XMLWriterTrait<'a> {
    fn new() -> XMLWriter<'a>;
    fn new_tag(
        &mut self,
        tag_name: &'a str,
        attributes: Vec<XMLAttributes>,
        content: Option<&str>
    );
    fn escape_characters(&mut self, content: String) -> String;
    fn close_current_tag(&mut self) -> Result<(), &'static str>;
}

impl<'a> XMLWriter<'a> {
    fn create_open_tag(
        &mut self,
        tag_name: &str,
        attributes: Vec<XMLAttributes>
    ) {
        self.left_pad();
        self.xml.push('<');
        self.xml.push_str(tag_name);
    
        for attr in attributes {
            self.xml.push_str(
                &format!(" {}=\"{}\"", attr.attribute_name, attr.attribute_value)
            );
        }
    
        self.xml.push('>');
    
        self.xml.push('\n');
    }
    
    fn add_content(&mut self, content: Option<&str>) {
        if let Some(c) = content {
            self.left_pad();
            self.xml.push_str(c);
            self.xml.push('\n');
        }
    }
    
    fn left_pad(&mut self) {
        self.xml.push_str(
            &"   ".repeat(self.last_padding)
        );
    }

    fn create_close_tag(&mut self, tag_name: &str) {
        self.left_pad();
        self.xml.push('<');
        self.xml.push_str(tag_name);
    
        self.xml.push_str("/>");
    
        self.xml.push('\n');
    }
}


impl<'a> XMLWriterTrait<'a> for XMLWriter<'a> {
    fn new() -> XMLWriter<'a> {
        XMLWriter {
            xml             : String::new(),
            last_padding    : 0,
            tags            : Vec::new(),
        }
    }

    fn new_tag(
        &mut self,
        tag_name: &'a str,
        attributes: Vec<XMLAttributes>,
        content: Option<&str>
    ) {
        self.create_open_tag(tag_name, attributes);
        self.add_content(content);

        self.tags.push(tag_name);
        
        self.last_padding += 1;
    }
    
    fn escape_characters(&mut self, content: String) -> String {
        content.replace("\"", "&quot;")
            .replace("'", "&apos;")
            .replace("<", "&lt;")
            .replace(">", "&gt;")
            .replace("&", "&amp;")
    }

    fn close_current_tag(&mut self) -> Result<(), &'static str> {
        self.last_padding -= 1;

        let tag = self.tags.last();

        if let Some(t) = tag {
            self.create_close_tag(t);
            return Ok(())
        } else {
            return Err("Invalid format: Close called too early");
        }
    }
}