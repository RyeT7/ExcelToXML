//! Conversion end to end: a session holding a table and its mappings, run
//! through the real XML writer, checked for the tags the formulas produce.

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use exceltoxml_lib::{
    application::{
        ports::{
            inbound::convertusecase::ConvertUseCase, outbound::sessionrepository::SessionRepository,
        },
        services::convertservice::ConvertService,
        session::session::Session,
    },
    domain::{
        datastructures::table::Table,
        entities::tagmapping::{TagMapping, TagMappings},
        enums::requiredtags::Tags,
    },
    infrastructure::adapters::outputs::{
        customxmlwriter::CustomXMLWriter, taurisessionrepository::TauriSessionRepository,
    },
};

const SESSION_ID: &str = "test-session";
const TIN: &str = "12123123";

/// One invoice with two goods, priced so the derived amounts cover both a
/// whole result and one that has to be rounded.
fn sample_table() -> Table {
    let headers = vec![
        "InvNo".to_string(),
        "Date".to_string(),
        "BuyerTin".to_string(),
        "Item".to_string(),
        "Price".to_string(),
        "Qty".to_string(),
        "VATRate".to_string(),
        "STLGRate".to_string(),
    ];

    let mut table = Table::new(&headers).unwrap();

    for (column, values) in [
        ("InvNo", ["IV1", "IV1"]),
        ("Date", ["2026-04-07", "2026-04-07"]),
        ("BuyerTin", ["1234", "1234"]),
        ("Item", ["A", "B"]),
        ("Price", ["100000", "1500.5"]),
        ("Qty", ["30", "3"]),
        ("VATRate", ["12", "11"]),
        ("STLGRate", ["", "5"]),
    ] {
        for value in values {
            table.push(column, value).unwrap();
        }
    }

    table
}

/// Mirrors what the mapping screen submits: every non-derived tag present, the
/// ones the formulas read pointed at real columns, the rest blank.
fn sample_mappings() -> TagMappings {
    let mut mappings: HashMap<String, TagMapping> = Tags::MAPPABLE
        .iter()
        .filter(|tag| !tag.is_derived())
        .map(|tag| {
            (
                tag.as_hierarchical_str().to_string(),
                TagMapping::new(None, Some(String::new())),
            )
        })
        .collect();

    for tag in [
        Tags::BuyerTin,
        Tags::Price,
        Tags::Qty,
        Tags::VATRate,
        Tags::STLGRate,
    ] {
        mappings.insert(
            tag.as_hierarchical_str().to_string(),
            TagMapping::new(Some(tag.as_literal_str().to_string()), None),
        );
    }

    mappings.insert(
        Tags::TaxInvoiceDate.as_hierarchical_str().to_string(),
        TagMapping::new(Some("Date".to_string()), None),
    );

    TagMappings {
        mappings,
        invoice_number_column: "InvNo".to_string(),
        good_service_identifier_column: "Item".to_string(),
    }
}

fn session_with(table: Table) -> Arc<TauriSessionRepository> {
    let session_repository = Arc::new(TauriSessionRepository::new());

    session_repository
        .insert(
            SESSION_ID,
            Session {
                table: Some(table),
                xml: None,
                tag_mappings: Some(sample_mappings()),
            },
        )
        .unwrap();

    session_repository
}

fn convert(session_repository: Arc<TauriSessionRepository>) -> Result<(), String> {
    ConvertService::new(
        session_repository,
        Arc::new(Mutex::new(CustomXMLWriter::new())),
    )
    .convert(SESSION_ID, TIN)
}

fn convert_sample() -> String {
    let session_repository = session_with(sample_table());

    convert(session_repository.clone()).unwrap();

    session_repository.get(SESSION_ID).unwrap().xml.unwrap()
}

#[test]
fn buyer_idtku_is_derived_from_the_buyer_tin() {
    let xml = convert_sample();

    assert!(xml.contains("<BuyerIDTKU>1234000000</BuyerIDTKU>"), "{xml}");
}

#[test]
fn dates_are_written_in_iso_form() {
    let xml = convert_sample();

    assert!(
        xml.contains("<TaxInvoiceDate>2026-04-07</TaxInvoiceDate>"),
        "{xml}"
    );
}

#[test]
fn an_unpadded_date_is_canonicalised_on_the_way_out() {
    let repository = session_with(with_column("Date", ["2026-4-7", "2026-4-7"]));

    convert(repository.clone()).unwrap();

    let xml = repository.get(SESSION_ID).unwrap().xml.unwrap();
    assert!(
        xml.contains("<TaxInvoiceDate>2026-04-07</TaxInvoiceDate>"),
        "{xml}"
    );
}

/// Text Excel never understood as a date has an unknowable day/month order, so
/// it is refused rather than guessed at.
#[test]
fn an_ambiguous_text_date_is_refused_with_its_location() {
    let error = convert(session_with(with_column(
        "Date",
        ["07/04/2026", "2026-04-07"],
    )))
    .unwrap_err();

    assert!(error.contains("TaxInvoiceDate"), "{error}");
    assert!(error.contains("07/04/2026"), "{error}");
    assert!(error.contains("row 2"), "{error}");
    assert!(error.contains("which part is the month"), "{error}");
}

#[test]
fn an_impossible_date_is_refused() {
    let error = convert(session_with(with_column(
        "Date",
        ["2026-02-30", "2026-04-07"],
    )))
    .unwrap_err();

    assert!(
        error.contains("TaxInvoiceDate") && error.contains("2026-02-30"),
        "{error}"
    );
}

#[test]
fn a_missing_date_is_refused() {
    let error = convert(session_with(with_column("Date", ["", "2026-04-07"]))).unwrap_err();

    assert!(
        error.contains("TaxInvoiceDate") && error.contains("is empty"),
        "{error}"
    );
}

#[test]
fn seller_idtku_is_derived_from_the_document_tin() {
    let xml = convert_sample();

    assert!(xml.contains(&format!("<TIN>{TIN}</TIN>")), "{xml}");
    assert!(
        xml.contains("<SellerIDTKU>12123123000000</SellerIDTKU>"),
        "{xml}"
    );
}

#[test]
fn amounts_are_derived_for_each_good_service() {
    let xml = convert_sample();

    // Item A: 100000 × 30, taxed at 12% with no STLG.
    for expected in [
        "<TaxBase>3000000</TaxBase>",
        "<OtherTaxBase>2750000</OtherTaxBase>",
        "<VAT>330000</VAT>",
        "<STLG>0</STLG>",
    ] {
        assert!(xml.contains(expected), "missing {expected} in:\n{xml}");
    }

    // Item B: 1500.5 × 3, where every step needs rounding.
    for expected in [
        "<TaxBase>4501.5</TaxBase>",
        "<OtherTaxBase>4126.38</OtherTaxBase>",
        "<VAT>453.9</VAT>",
        "<STLG>206.32</STLG>",
    ] {
        assert!(xml.contains(expected), "missing {expected} in:\n{xml}");
    }
}

/// Replaces a whole column, keeping the two rows aligned.
fn with_column(column: &str, values: [&str; 2]) -> Table {
    let mut table = sample_table();

    table.data.insert(
        column.to_string(),
        values.iter().map(|value| value.to_string()).collect(),
    );

    table
}

#[test]
fn a_non_numeric_amount_names_the_tag_column_row_and_invoice() {
    let session_repository = session_with(with_column("Qty", ["n/a", "3"]));

    let error = convert(session_repository.clone()).unwrap_err();

    assert!(error.contains("Qty"), "{error}");
    assert!(error.contains("n/a"), "{error}");
    assert!(error.contains("column \"Qty\""), "{error}");
    // Header is row 1, so the first data row is row 2.
    assert!(error.contains("row 2"), "{error}");
    assert!(error.contains("invoice \"IV1\""), "{error}");

    // A failed conversion must not leave a downloadable document behind.
    assert!(session_repository.get(SESSION_ID).unwrap().xml.is_none());
}

#[test]
fn every_bad_cell_is_reported_in_one_pass() {
    let mut table = with_column("Qty", ["n/a", "3"]);
    table.data.insert(
        "Price".to_string(),
        vec!["100000".to_string(), "TBD".to_string()],
    );

    let error = convert(session_with(table)).unwrap_err();

    assert!(
        error.starts_with("2 cells in the file cannot be used"),
        "{error}"
    );
    assert!(error.contains("Qty") && error.contains("n/a"), "{error}");
    assert!(error.contains("Price") && error.contains("TBD"), "{error}");
}

#[test]
#[ignore = "prints a sample report for eyeballing the wording"]
fn preview_report() {
    let mut table = with_column("Qty", ["n/a", ""]);
    table.data.insert(
        "Price".to_string(),
        vec!["Rp 100.000".to_string(), "1500.5".to_string()],
    );
    table.data.insert(
        "VATRate".to_string(),
        vec!["12%".to_string(), "11".to_string()],
    );

    println!("\n{}\n", convert(session_with(table)).unwrap_err());
}

#[test]
fn likely_mistakes_come_with_a_hint() {
    let error = convert(session_with(with_column("VATRate", ["12%", "11"]))).unwrap_err();

    assert!(error.contains("12%"), "{error}");
    assert!(error.contains("drop the % sign"), "{error}");
}

/// A blank rate means the seller does not charge it; a blank price or quantity
/// is a hole in the data and must not quietly become a zero-value line.
#[test]
fn blank_rates_are_allowed_but_blank_prices_are_not() {
    let xml_repository = session_with(with_column("STLGRate", ["", ""]));

    convert(xml_repository.clone()).unwrap();

    let xml = xml_repository.get(SESSION_ID).unwrap().xml.unwrap();
    assert!(xml.contains("<STLG>0</STLG>"), "{xml}");

    let error = convert(session_with(with_column("Price", ["", "1500.5"]))).unwrap_err();

    assert!(error.contains("Price"), "{error}");
    assert!(error.contains("is empty"), "{error}");

    let error = convert(session_with(with_column("Qty", ["30", ""]))).unwrap_err();

    assert!(
        error.contains("Qty") && error.contains("is empty"),
        "{error}"
    );
}

#[test]
fn a_non_numeric_default_value_is_reported_without_a_row() {
    let session_repository = Arc::new(TauriSessionRepository::new());

    let mut tag_mappings = sample_mappings();
    tag_mappings.mappings.insert(
        Tags::VATRate.as_hierarchical_str().to_string(),
        TagMapping::new(None, Some("twelve".to_string())),
    );

    session_repository
        .insert(
            SESSION_ID,
            Session {
                table: Some(sample_table()),
                xml: None,
                tag_mappings: Some(tag_mappings),
            },
        )
        .unwrap();

    let error = convert(session_repository).unwrap_err();

    assert!(error.contains("VATRate"), "{error}");
    assert!(error.contains("default value \"twelve\""), "{error}");
}
