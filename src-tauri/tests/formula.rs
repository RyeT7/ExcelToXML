//! Arithmetic behind the derived tags: parsing cell values, rounding, and the
//! five formulas themselves.

use std::str::FromStr;

use rust_decimal::Decimal;

use exceltoxml_lib::domain::services::formula;

fn amount(value: &str) -> Decimal {
    Decimal::from_str(value).unwrap()
}

#[test]
fn blank_amounts_are_reported_separately_from_unreadable_ones() {
    // The caller decides whether blank is acceptable, so the two cases stay
    // distinguishable.
    assert_eq!(
        formula::parse_amount("   "),
        Err(formula::AmountError::Blank)
    );
    assert_eq!(
        formula::parse_amount("n/a"),
        Err(formula::AmountError::NotANumber)
    );
}

#[test]
fn surrounding_whitespace_is_tolerated() {
    assert_eq!(formula::parse_amount("  1500.5  "), Ok(amount("1500.5")));
}

#[test]
fn common_spreadsheet_mistakes_get_a_hint() {
    assert!(formula::amount_hint("12%").unwrap().contains("% sign"));
    assert!(formula::amount_hint("1.234.567")
        .unwrap()
        .contains("thousands separators"));
    assert!(formula::amount_hint("1.500,50")
        .unwrap()
        .contains("decimal point"));

    // Following a bare "remove the currency symbol" on a grouped amount would
    // leave "100.000", which reads as 100. The hint has to cover both.
    let hint = formula::amount_hint("Rp 100.000").unwrap();
    assert!(hint.contains("currency symbol"), "{hint}");
    assert!(hint.contains("thousands separators"), "{hint}");

    // A plain unreadable value has no obvious cause to point at.
    assert_eq!(formula::amount_hint("--"), None);
}

#[test]
fn whole_amounts_drop_their_decimals() {
    assert_eq!(formula::format_amount(amount("3000000.00")), "3000000");
    assert_eq!(formula::format_amount(Decimal::ZERO), "0");
}

#[test]
fn fractional_amounts_keep_two_decimals() {
    assert_eq!(formula::format_amount(amount("2750000.125")), "2750000.13");
    assert_eq!(formula::format_amount(amount("1.5")), "1.5");
}

#[test]
fn idtku_appends_the_suffix() {
    assert_eq!(formula::idtku("1234"), "1234000000");
    assert_eq!(formula::idtku("12123123"), "12123123000000");
}

#[test]
fn amounts_chain_from_the_rounded_previous_value() {
    let tax_base = formula::tax_base(amount("100000"), amount("30"));
    let other_tax_base = formula::other_tax_base(tax_base);

    assert_eq!(formula::format_amount(tax_base), "3000000");
    assert_eq!(formula::format_amount(other_tax_base), "2750000");
    assert_eq!(
        formula::format_amount(formula::vat(other_tax_base, amount("12"))),
        "330000"
    );
    assert_eq!(
        formula::format_amount(formula::stlg(other_tax_base, Decimal::ZERO)),
        "0"
    );
}

/// Exact half-way values are the whole reason this is decimal arithmetic. Each
/// case below is one that binary floating point rounds the wrong way.
#[test]
fn exact_half_cents_round_away_from_zero() {
    // 82.5 × 11 ÷ 100 = 9.075 exactly. f64 gives 9.07.
    assert_eq!(
        formula::format_amount(formula::vat(amount("82.5"), amount("11"))),
        "9.08"
    );

    // 1182.5 × 11 ÷ 100 = 130.075 exactly. f64 gives 130.07.
    assert_eq!(
        formula::format_amount(formula::vat(amount("1182.5"), amount("11"))),
        "130.08"
    );

    // 0.3 × 11 ÷ 12 = 0.275 exactly. f64 gives 0.27.
    assert_eq!(
        formula::format_amount(formula::other_tax_base(amount("0.3"))),
        "0.28"
    );

    // The textbook case: 1.005 to two places.
    assert_eq!(formula::format_amount(amount("1.005")), "1.01");
}

/// The whole chain for `price=90, qty=1, rate=11%`, which the f64 version got
/// wrong by a cent.
#[test]
fn a_plain_ninety_rupiah_line_is_exact() {
    let tax_base = formula::tax_base(amount("90"), amount("1"));
    let other_tax_base = formula::other_tax_base(tax_base);

    assert_eq!(formula::format_amount(tax_base), "90");
    assert_eq!(formula::format_amount(other_tax_base), "82.5");
    assert_eq!(
        formula::format_amount(formula::vat(other_tax_base, amount("11"))),
        "9.08"
    );
}

/// Decimal cell values that have no exact binary representation.
#[test]
fn prices_with_cents_stay_exact() {
    let tax_base = formula::tax_base(amount("1500.10"), amount("3"));

    // f64 computes this as 4500.299999999999.
    assert_eq!(formula::format_amount(tax_base), "4500.3");
}

/// IDR line totals run large; the amounts must not lose precision at scale.
#[test]
fn large_amounts_keep_full_precision() {
    let tax_base = formula::tax_base(amount("1234567890"), amount("1000"));

    assert_eq!(formula::format_amount(tax_base), "1234567890000");
}
