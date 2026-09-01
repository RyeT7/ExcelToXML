//! Dates are written to the document in ISO form, `YYYY-MM-DD`.
//!
//! Excel already settled how a date was meant to be read at the moment it was
//! typed: a real date cell stores one specific day, and `07/04/2026` is only
//! how that day is displayed. So the reader converts date cells through here
//! using the cell's type, and no part of the program ever has to guess whether
//! a day or a month came first.
//!
//! The consequence is worth stating plainly: once a value has been through the
//! reader, a correct date is already ISO. Anything that is *not* ISO by then
//! was text that Excel itself never recognised as a date — which is exactly
//! the case where the day/month order is unknowable and must be refused
//! rather than guessed at.

use chrono::{NaiveDate, NaiveDateTime};

/// The only date format this program reads or writes.
pub const ISO_FORMAT: &str = "%Y-%m-%d";

/// Renders a date cell in the form the document uses.
pub fn to_iso(value: NaiveDateTime) -> String {
    value.format(ISO_FORMAT).to_string()
}

/// Reads a value that should already be ISO.
///
/// Rejects both the wrong shape and impossible days such as `2026-02-30`.
pub fn parse_iso(value: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(value.trim(), ISO_FORMAT).ok()
}

/// Rewrites an ISO value in its canonical form, so `2026-4-7` is padded to
/// `2026-04-07`. Returns `None` if the value is not a date at all.
pub fn normalize(value: &str) -> Option<String> {
    parse_iso(value).map(|date| date.format(ISO_FORMAT).to_string())
}

/// Reduces an ISO 8601 timestamp such as `2026-04-07T00:00:00` to its date.
///
/// A value that cannot be read is handed back untouched, to be reported later
/// against the tag that needed it rather than silently blanked here.
pub fn normalize_timestamp(value: &str) -> String {
    value
        .split('T')
        .next()
        .and_then(normalize)
        .unwrap_or_else(|| value.to_string())
}
