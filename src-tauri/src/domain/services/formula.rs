//! Values for the derived tags are computed here instead of being read from
//! the uploaded file.
//!
//! These are monetary amounts, so the arithmetic is decimal, not binary
//! floating point. The formulas land on exact half-way values often enough to
//! matter: whenever TaxBase ≡ 6 (mod 12), OtherTaxBase ends in `.5`, and an
//! 11% rate on that gives an exact half-cent — `82.5 × 11 ÷ 100 = 9.075`.
//! `f64` stores that as `9.07499…` and rounds it down to `9.07` instead of up
//! to `9.08`. Roughly 1% of ordinary whole-rupiah line items hit this.
//!
//! Every amount is also rounded as soon as it is produced, and the rounded
//! value is what feeds the next formula. That way the numbers written into the
//! document agree with each other: recomputing VAT from the `<OtherTaxBase>`
//! that was actually written gives back the `<VAT>` that was actually written.

use std::str::FromStr;

use rust_decimal::{Decimal, RoundingStrategy};

/// Decimal places kept in derived amounts.
const SCALE: u32 = 2;

/// Appended to a TIN to form the matching IDTKU.
const IDTKU_SUFFIX: &str = "000000";

/// Rounds half away from zero to [`SCALE`] decimals, which is the direction
/// tax rounding is expected to take (and, for these non-negative amounts, the
/// same thing as rounding half up).
fn round_amount(value: Decimal) -> Decimal {
    value.round_dp_with_strategy(SCALE, RoundingStrategy::MidpointAwayFromZero)
}

fn constant(value: u8) -> Decimal {
    Decimal::from(value)
}

/// Why a cell could not be read as an amount.
///
/// Blank is kept separate from unreadable because whether an empty cell is
/// acceptable depends on the tag: a rate that is not charged may be left
/// empty, a price may not.
#[derive(Debug, PartialEq, Eq)]
pub enum AmountError {
    Blank,
    NotANumber,
}

/// Parses a cell value into an amount.
pub fn parse_amount(value: &str) -> Result<Decimal, AmountError> {
    let trimmed = value.trim();

    if trimmed.is_empty() {
        return Err(AmountError::Blank);
    }

    Decimal::from_str(trimmed).map_err(|_| AmountError::NotANumber)
}

/// The likely cause of an unreadable cell, when the value looks like one of
/// the usual spreadsheet mistakes.
///
/// These are only ever shown as advice — a value is never silently repaired,
/// because guessing at what a tax figure was meant to say is worse than
/// refusing it.
pub fn amount_hint(value: &str) -> Option<&'static str> {
    let trimmed = value.trim();

    if trimmed.contains('%') {
        return Some("drop the % sign; a rate is written as a plain number, so 12% is 12");
    }

    // Grouped thousands are the dangerous case: strip only the currency symbol
    // from "Rp 100.000" and what is left parses as 100, so every hint that
    // could apply to such a value has to mention the separators too.
    if trimmed.chars().any(|c| c.is_ascii_alphabetic()) {
        return Some("leave only the number, with no currency symbol and no thousands separators");
    }

    if trimmed.contains(',') || trimmed.matches('.').count() > 1 {
        return Some("write it without thousands separators and with \".\" for the decimal point, so 1.234.567,89 becomes 1234567.89");
    }

    None
}

/// Renders an amount for the document, dropping trailing zeros so whole
/// amounts stay written as integers.
pub fn format_amount(value: Decimal) -> String {
    let rounded = round_amount(value);

    if rounded.is_zero() {
        // Also collapses a negative zero, which would render as "-0".
        return "0".to_string();
    }

    rounded.normalize().to_string()
}

/// `SellerIDTKU = CONCAT(TIN, "000000")` and
/// `BuyerIDTKU = CONCAT(BuyerTin, "000000")` — the same concatenation, over
/// the seller's TIN and the buyer's respectively.
pub fn idtku(tin: &str) -> String {
    format!("{}{}", tin.trim(), IDTKU_SUFFIX)
}

/// `TaxBase = Price * Qty`
pub fn tax_base(price: Decimal, qty: Decimal) -> Decimal {
    round_amount(price * qty)
}

/// `OtherTaxBase = (TaxBase * 11) / 12`
pub fn other_tax_base(tax_base: Decimal) -> Decimal {
    round_amount((tax_base * constant(11)) / constant(12))
}

/// `VAT = (OtherTaxBase * VATRate) / 100`
pub fn vat(other_tax_base: Decimal, vat_rate: Decimal) -> Decimal {
    round_amount((other_tax_base * vat_rate) / constant(100))
}

/// `STLG = (OtherTaxBase * STLGRate) / 100`
pub fn stlg(other_tax_base: Decimal, stlg_rate: Decimal) -> Decimal {
    round_amount((other_tax_base * stlg_rate) / constant(100))
}
