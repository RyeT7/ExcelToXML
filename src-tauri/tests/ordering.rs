//! Invoices and their goods must appear in the document in the order the
//! spreadsheet lists them.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

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

const SESSION_ID: &str = "ordering";

/// Ten invoices, one line each, listed IV01 through IV10.
fn ordered_table() -> Table {
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

    for row in 1..=10 {
        table.push("InvNo", &format!("IV{row:02}")).unwrap();
        table.push("Date", "2026-04-07").unwrap();
        table.push("BuyerTin", "1234").unwrap();
        table.push("Item", &format!("ITEM{row:02}")).unwrap();
        table.push("Price", "100000").unwrap();
        table.push("Qty", "1").unwrap();
        table.push("VATRate", "12").unwrap();
        table.push("STLGRate", "0").unwrap();
    }

    table
}

fn mappings() -> TagMappings {
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

    for (tag, column) in [(Tags::TaxInvoiceDate, "Date"), (Tags::Name, "Item")] {
        mappings.insert(
            tag.as_hierarchical_str().to_string(),
            TagMapping::new(Some(column.to_string()), None),
        );
    }

    // RefDesc carries the invoice number so the order is visible in the output.
    mappings.insert(
        Tags::RefDesc.as_hierarchical_str().to_string(),
        TagMapping::new(Some("InvNo".to_string()), None),
    );

    TagMappings {
        mappings,
        invoice_number_column: "InvNo".to_string(),
        good_service_identifier_column: "Item".to_string(),
    }
}

fn invoice_order() -> Vec<String> {
    let repository = Arc::new(TauriSessionRepository::new());

    repository
        .insert(
            SESSION_ID,
            Session {
                table: Some(ordered_table()),
                xml: None,
                tag_mappings: Some(mappings()),
            },
        )
        .unwrap();

    ConvertService::new(
        repository.clone(),
        Arc::new(Mutex::new(CustomXMLWriter::new())),
    )
    .convert(SESSION_ID, "12123123")
    .unwrap();

    repository
        .get(SESSION_ID)
        .unwrap()
        .xml
        .unwrap()
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            line.strip_prefix("<RefDesc>")
                .and_then(|rest| rest.strip_suffix("</RefDesc>"))
                .map(str::to_string)
        })
        .collect()
}

/// Reads the goods of the single invoice in `table`, in document order.
fn good_service_order(table: Table) -> Vec<String> {
    let repository = Arc::new(TauriSessionRepository::new());

    repository
        .insert(
            SESSION_ID,
            Session {
                table: Some(table),
                xml: None,
                tag_mappings: Some(mappings()),
            },
        )
        .unwrap();

    ConvertService::new(
        repository.clone(),
        Arc::new(Mutex::new(CustomXMLWriter::new())),
    )
    .convert(SESSION_ID, "12123123")
    .unwrap();

    repository
        .get(SESSION_ID)
        .unwrap()
        .xml
        .unwrap()
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            line.strip_prefix("<Name>")
                .and_then(|rest| rest.strip_suffix("</Name>"))
                .map(str::to_string)
        })
        .collect()
}

#[test]
fn invoices_keep_the_order_the_spreadsheet_lists_them_in() {
    let expected: Vec<String> = (1..=10).map(|row| format!("IV{row:02}")).collect();

    assert_eq!(invoice_order(), expected);
}

/// Grouping happens a second time inside each invoice, so the goods need the
/// same guarantee as the invoices around them.
#[test]
fn goods_keep_the_order_the_spreadsheet_lists_them_in() {
    let mut table = ordered_table();

    // Collapse the ten rows onto one invoice, leaving ten distinct goods.
    table.data.insert(
        "InvNo".to_string(),
        std::iter::repeat_n("IV01".to_string(), 10).collect(),
    );

    let expected: Vec<String> = (1..=10).map(|row| format!("ITEM{row:02}")).collect();

    assert_eq!(good_service_order(table), expected);
}

/// The same file converted twice has to produce the same document.
#[test]
fn conversion_is_reproducible() {
    assert_eq!(invoice_order(), invoice_order());
}
