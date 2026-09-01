//! Dates reach the document in ISO form, and only ever by a route that does
//! not guess at day/month order.

use calamine::{Data, ExcelDateTime, ExcelDateTimeType};

use exceltoxml_lib::domain::services::dates;

#[test]
fn iso_values_are_accepted() {
    assert_eq!(
        dates::normalize("2026-04-07").as_deref(),
        Some("2026-04-07")
    );
}

#[test]
fn unpadded_iso_values_are_canonicalised() {
    // Unambiguous whichever way it is read, so it is padded rather than refused.
    assert_eq!(dates::normalize("2026-4-7").as_deref(), Some("2026-04-07"));
}

#[test]
fn surrounding_whitespace_is_tolerated() {
    assert_eq!(
        dates::normalize("  2026-04-07 ").as_deref(),
        Some("2026-04-07")
    );
}

#[test]
fn impossible_days_are_refused() {
    assert_eq!(dates::normalize("2026-02-30"), None);
    assert_eq!(dates::normalize("2026-13-01"), None);
}

/// The whole point: a date whose day/month order cannot be known is refused
/// rather than guessed at.
#[test]
fn ambiguous_text_dates_are_refused() {
    assert_eq!(dates::normalize("07/04/2026"), None);
    assert_eq!(dates::normalize("04/07/2026"), None);
    assert_eq!(dates::normalize("07-Apr-26"), None);
}

#[test]
fn iso_timestamps_are_reduced_to_their_date() {
    assert_eq!(
        dates::normalize_timestamp("2026-04-07T00:00:00"),
        "2026-04-07"
    );
    assert_eq!(dates::normalize_timestamp("2026-04-07"), "2026-04-07");
}

#[test]
fn an_unreadable_timestamp_is_left_for_the_caller_to_report() {
    assert_eq!(dates::normalize_timestamp("not a date"), "not a date");
}

/// Excel stores a date as a serial number carrying a date format, and
/// `ExcelDateTime`'s own `Display` writes that raw number. Reading the cell's
/// type instead is what turns it back into a day.
#[test]
fn an_excel_date_cell_becomes_an_iso_date_not_a_serial_number() {
    // 46114 is 2026-04-02 in Excel's 1900 date system.
    let cell = Data::DateTime(ExcelDateTime::new(
        46114.0,
        ExcelDateTimeType::DateTime,
        false,
    ));

    // What the old code wrote straight into <TaxInvoiceDate>.
    let serial = cell.to_string();
    assert!(serial.starts_with("46114"), "{serial}");

    let datetime = match &cell {
        Data::DateTime(value) => value.as_datetime().unwrap(),
        _ => unreachable!(),
    };

    assert_eq!(dates::to_iso(datetime), "2026-04-02");
}
