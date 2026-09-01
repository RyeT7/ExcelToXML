//! Text taken from the spreadsheet has to survive into the document and back
//! out of a parser unchanged.

use exceltoxml_lib::{
    application::ports::outbound::xmlwriter::XMLWriter,
    domain::enums::xmlattributes::XMLAttributes,
    infrastructure::adapters::outputs::customxmlwriter::CustomXMLWriter,
};

fn write_content(content: &str) -> String {
    let mut writer = CustomXMLWriter::new();

    writer.new_open_close_tag("Name", &[], Some(content));

    writer.take_xml()
}

/// The reported case: a quotation mark in a product name.
#[test]
fn quotation_marks_are_left_as_typed() {
    let name = "Check Valve One Way InFlow 1/2\" SS304 Body + Plastic Insert";

    assert_eq!(write_content(name), format!("<Name>{name}</Name>\n"));
}

#[test]
fn apostrophes_are_left_as_typed() {
    assert_eq!(write_content("O'Ring 3/4\""), "<Name>O'Ring 3/4\"</Name>\n");
}

/// `&`, `<` and `>` are the only characters that mean markup between tags, so
/// they are the only ones escaped.
#[test]
fn markup_characters_are_escaped_exactly_once() {
    assert_eq!(
        write_content("Nuts & Bolts"),
        "<Name>Nuts &amp; Bolts</Name>\n"
    );
    assert_eq!(
        write_content("Bracket <L> type"),
        "<Name>Bracket &lt;L&gt; type</Name>\n"
    );
}

/// Escaping `&` last would rewrite the `&` of each entity just produced,
/// turning `<` into `&amp;lt;` — which reads back as the text `&lt;`.
#[test]
fn entities_are_not_escaped_a_second_time() {
    let written = write_content("A & B <C> \"D\"");

    assert_eq!(written, "<Name>A &amp; B &lt;C&gt; \"D\"</Name>\n");
    assert!(!written.contains("&amp;lt;"), "{written}");
    assert!(!written.contains("&amp;quot;"), "{written}");
}

/// Text that already looks like an entity is data, not markup, and has to come
/// back out exactly as it went in.
#[test]
fn text_resembling_an_entity_survives_a_round_trip() {
    let written = write_content("literally &amp; here");

    assert_eq!(written, "<Name>literally &amp;amp; here</Name>\n");
}

/// An attribute value is delimited by quotes, so there `"` does need escaping.
#[test]
fn attribute_values_escape_their_delimiter() {
    let mut writer = CustomXMLWriter::new();

    writer.new_open_close_tag(
        "Tag",
        &[XMLAttributes {
            attribute_name: "note".to_string(),
            attribute_value: "1/2\" & <wide>".to_string(),
        }],
        Some("body"),
    );

    assert_eq!(
        writer.take_xml(),
        "<Tag note=\"1/2&quot; &amp; &lt;wide&gt;\">body</Tag>\n"
    );
}

/// The namespace attributes the document opens with are unaffected.
#[test]
fn ordinary_attribute_values_are_untouched() {
    let mut writer = CustomXMLWriter::new();

    writer.new_open_close_tag(
        "TaxInvoiceBulk",
        &[XMLAttributes {
            attribute_name: "xmlns:xsd".to_string(),
            attribute_value: "http://www.w3.org/2001/XMLSchema".to_string(),
        }],
        Some("x"),
    );

    assert!(
        writer
            .take_xml()
            .contains("xmlns:xsd=\"http://www.w3.org/2001/XMLSchema\""),
        "namespace declaration should be written verbatim"
    );
}
