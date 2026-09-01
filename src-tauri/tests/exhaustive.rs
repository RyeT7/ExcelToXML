//! Brute-force check of the derived amounts against integer arithmetic done in
//! whole cents, which cannot round wrong.
//!
//! Slow, so it is ignored by default. Run it after touching `formula`:
//!
//! ```text
//! cargo test --release --test exhaustive -- --ignored --nocapture
//! ```

use std::str::FromStr;

use rust_decimal::Decimal;

use exceltoxml_lib::domain::services::formula;

/// Integer division rounding half away from zero — the reference the formulas
/// have to match.
fn div_round(numerator: i128, denominator: i128) -> i128 {
    match numerator >= 0 {
        true => (2 * numerator + denominator) / (2 * denominator),
        false => -((-2 * numerator + denominator) / (2 * denominator)),
    }
}

fn cents(value: Decimal) -> i128 {
    i128::from_str(&formula::format_amount(value * Decimal::from(100)))
        .expect("a rounded amount times 100 is a whole number")
}

#[test]
#[ignore = "exhaustive sweep; run explicitly in release"]
fn derived_amounts_match_integer_cent_arithmetic() {
    let mut checked = 0u64;

    for price in 1i128..=2_000_000 {
        for qty in [1i128, 3, 6, 18, 30] {
            for rate in [11i128, 12] {
                checked += 1;

                // Reference, entirely in whole cents.
                let expected_tax_base = price * 100 * qty;
                let expected_other = div_round(expected_tax_base * 11, 12);
                let expected_vat = div_round(expected_other * rate, 100);

                // The real implementation.
                let tax_base = formula::tax_base(Decimal::from(price), Decimal::from(qty));
                let other_tax_base = formula::other_tax_base(tax_base);
                let vat = formula::vat(other_tax_base, Decimal::from(rate));

                assert_eq!(
                    cents(tax_base),
                    expected_tax_base,
                    "TaxBase for price={price} qty={qty}"
                );
                assert_eq!(
                    cents(other_tax_base),
                    expected_other,
                    "OtherTaxBase for price={price} qty={qty}"
                );
                assert_eq!(
                    cents(vat),
                    expected_vat,
                    "VAT for price={price} qty={qty} rate={rate}%"
                );
            }
        }
    }

    println!("{checked} combinations matched exact cent arithmetic");
}
