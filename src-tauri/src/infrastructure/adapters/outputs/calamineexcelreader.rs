use calamine::{open_workbook_auto, Data, Range, Reader};

use crate::{
    application::ports::outbound::excelreader::ExcelReader,
    domain::{datastructures::table::Table, services::dates},
};

pub struct CalamineExcelReader;

impl CalamineExcelReader {
    pub fn new() -> Self {
        Self {}
    }

    /// Renders a cell as the text the rest of the program works with.
    ///
    /// Date cells are the reason this is not just `to_string`. Excel stores a
    /// date as a number carrying a date format, and `ExcelDateTime`'s own
    /// `Display` writes that raw serial number — so a date cell used to arrive
    /// as `46114`. Going through the cell's *type* recovers the day it stands
    /// for, which also settles how the value was meant to be read: whatever
    /// the sheet displays, Excel knows which part is the month.
    fn cell_to_string(cell: &Data) -> String {
        match cell {
            Data::String(s) => s.to_string(),
            Data::DateTime(d) => match d.as_datetime() {
                Some(datetime) => dates::to_iso(datetime),
                // A duration rather than a point in time; leave it for the
                // tag that reads it to reject.
                None => d.to_string(),
            },
            Data::DateTimeIso(s) => dates::normalize_timestamp(s),
            Data::Float(f) => f.to_string(),
            Data::Int(i) => i.to_string(),
            _ => String::from(""),
        }
    }

    fn extract_header(range: &Range<Data>) -> Result<Vec<String>, String> {
        let header_row = match range.rows().into_iter().next() {
            Some(r) => r,
            None => return Err("Header rows not found".to_string()),
        };

        let mut headers: Vec<String> = Vec::new();

        for cell in header_row.iter() {
            headers.push(Self::cell_to_string(cell));
        }

        Ok(headers)
    }

    fn read_excel(header: &Vec<String>, range: &Range<Data>) -> Result<Table, String> {
        let mut table = Table::new(header)?;

        for row in range.rows().skip(1) {
            for (col_idx, cell) in row.iter().enumerate() {
                let header = match header.get(col_idx) {
                    Some(h) => h,
                    None => {
                        return Err(format!("Column {} is missing", col_idx));
                    }
                };

                let cell_value = Self::cell_to_string(cell);

                match table.push(header, &cell_value) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                };
            }
        }

        Ok(table)
    }
}

impl ExcelReader for CalamineExcelReader {
    fn load_excel(&self, path: &str) -> Result<Table, String> {
        let workbook = open_workbook_auto(path);

        let range = match workbook {
            Ok(mut w) => match w.worksheet_range_at(0) {
                Some(Ok(range)) => range,
                Some(Err(_)) => {
                    return Err("Something went wrong: sheet not found in workbook".to_string())
                }
                None => return Err("There are no sheets on this workbook".to_string()),
            },
            Err(_) => {
                return Err("Workbook does not exist".to_string());
            }
        };

        let header = Self::extract_header(&range)?;

        let table = Self::read_excel(&header, &range)?;

        Ok(table)
    }
}
