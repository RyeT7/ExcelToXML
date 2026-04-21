use crate::xml::attributes::XMLAttributes;

pub struct XMLWriter<'a> {
    xml: String,
    tags: Vec<&'a str>,
    last_padding: usize,
}

pub trait XMLWriterTrait<'a> {
    fn new() -> XMLWriter<'a>;
    fn new_open_tag(
        &mut self,
        tag_name: &'a str,
        attributes: &[XMLAttributes],
        content: Option<&str>
    );
    fn escape_characters(&mut self, content: &str) -> String;
    fn close_current_tag(&mut self) -> Result<(), &'static str>;
    fn new_empty_tag(&mut self, tag_name: &str) -> Result<(), &'static str>;
    fn new_open_close_tag(
        &mut self,
        tag_name: &str,
        attributes: &[XMLAttributes],
        content: Option<&str>
    );
    fn new_open_close_tag_no_attributes(
        &mut self,
        tag_name: &str,
        content: Option<&str>
    );
}

impl<'a> XMLWriter<'a> {
    fn create_open_tag(
        &mut self,
        tag_name: &str,
        attributes: &[XMLAttributes]
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

            let normalized_content = self.escape_characters(c);

            self.xml.push_str(
                &normalized_content
            );

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
        self.xml.push_str("</");
        self.xml.push_str(tag_name);
    
        self.xml.push('>');
    
        self.xml.push('\n');
    }

    fn create_self_closing_tag(&mut self, tag_name: &str) {
        self.left_pad();
        self.xml.push_str(&format!("<{} />\n", tag_name));
    }

    fn create_one_line_open_close_tag(
        &mut self,
        tag_name: &str,
        attributes: &[XMLAttributes],
        content: Option<&str>
    ) {
        self.left_pad();
        self.xml.push('<');
        self.xml.push_str(tag_name);

        if attributes.len() > 0 {
            for attr in attributes {
                self.xml.push_str(
                    &format!(" {}=\"{}\"", attr.attribute_name, attr.attribute_value)
                );
            }
        }

        match content {
            Some(c) => {
                let normalized_content = self.escape_characters(c);
                self.xml.push_str(
                    &format!(">{}</{}>\n", normalized_content, tag_name)
                );
            },
            None => {
                self.xml.push_str(
                    &format!("></{}>\n", tag_name)
                );
            },
        }
    }

    pub fn get_xml(&self) -> &str {
        &self.xml 
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

    fn new_open_tag(
        &mut self,
        tag_name: &'a str,
        attributes: &[XMLAttributes],
        content: Option<&str>
    ) {
        self.create_open_tag(tag_name, attributes);
        self.add_content(content);

        self.tags.push(tag_name);
        
        self.last_padding += 1;
    }
    
    fn escape_characters(&mut self, content: &str) -> String {
        content.replace("\"", "&quot;")
            .replace("'", "&apos;")
            .replace("<", "&lt;")
            .replace(">", "&gt;")
            .replace("&", "&amp;")
    }

    fn close_current_tag(&mut self) -> Result<(), &'static str> {
        self.last_padding -= 1;

        if let Some(t) = self.tags.pop() {
            self.create_close_tag(t);
            return Ok(())
        } else {
            return Err("Invalid format: Close called too early");
        }
    }

    fn new_empty_tag(&mut self, tag_name: &str) -> Result<(), &'static str> {
        self.create_self_closing_tag(tag_name);

        self.last_padding -= 1;

        Ok(())
    }

    /// This function can either create an immediate open and close tag
    /// or it could create a self closing tag depending on what your
    /// content is.
    /// 
    /// A non-empty string will get you an open tag, however, None or an
    /// empty string will produce a self closing tag.
    /// 
    /// # Examples
    /// ```
    /// extern crate exceltoxml_lib;
    /// 
    /// use exceltoxml_lib::xml::writer::{XMLWriter, XMLWriterTrait};
    /// 
    /// let mut xml_writer = XMLWriter::new();
    /// 
    /// xml_writer.new_open_close_tag("Tag", &[], None);
    /// 
    /// xml_writer.new_open_close_tag("Tag", &[], Some("content"));
    /// 
    /// assert_eq!(
    ///     xml_writer.get_xml(),
    ///     "<Tag />\n<Tag>content</Tag>\n"
    /// );
    /// 
    /// ```
    fn new_open_close_tag(
        &mut self,
        tag_name: &str,
        attributes: &[XMLAttributes],
        content: Option<&str>
    ) {
        if content.is_some_and(|c| !c.is_empty()) {
            self.create_one_line_open_close_tag(tag_name, attributes, content);
            return;
        }

        self.create_self_closing_tag(tag_name);
    }

    fn new_open_close_tag_no_attributes(
        &mut self,
        tag_name: &str,
        content: Option<&str>
    ) {
        self.new_open_close_tag(
            tag_name,
            &[],
            content
        );
    }

}