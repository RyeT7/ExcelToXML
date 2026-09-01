use crate::domain::enums::xmlattributes::XMLAttributes;

pub trait XMLWriter: Send + Sync {
    fn new_open_tag(&mut self, tag_name: &str, attributes: &[XMLAttributes], content: Option<&str>);
    fn escape_characters(&mut self, content: &str) -> String;
    fn close_current_tag(&mut self) -> Result<(), String>;
    fn new_empty_tag(&mut self, tag_name: &str) -> Result<(), String>;
    fn new_open_close_tag(
        &mut self,
        tag_name: &str,
        attributes: &[XMLAttributes],
        content: Option<&str>,
    );
    fn new_open_close_tag_no_attributes(&mut self, tag_name: &str, content: Option<&str>);
    /// Returns the XML built so far and resets the writer back to its
    /// initial empty state, ready to build the next document.
    fn take_xml(&mut self) -> String;
}
