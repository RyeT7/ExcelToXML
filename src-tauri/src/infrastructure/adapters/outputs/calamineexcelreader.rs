use calamine::{Data, Range, Reader, open_workbook_auto};

use crate::{application::ports::outbound::excelreader::ExcelReader, domain::datastructures::table::Table};

pub struct CalamineExcelReader;

impl CalamineExcelReader {
    pub fn new() -> Self {
        Self {  }
    }

    fn extract_header(range: &Range<Data>) -> Result<Vec<String>, String> {
        let header_row = match range.rows().into_iter().next() {
            Some(r) => r,
            None => {
                return Err("Header rows not found".to_string())
            }
        };

        let mut headers: Vec<String> = Vec::new();

        for cell in header_row.iter() {
            let cell_value = match cell {
                Data::String(s) => s.to_string(),
                Data::DateTime(d) => d.to_string(),
                Data::Float(f) => f.to_string(),
                Data::Int(i) => i.to_string(),
                _ => String::from("")
            };

            headers.push(cell_value);
        }

        Ok(headers)
    }

    fn read_excel(
        header: &Vec<String>,
        range: &Range<Data>
    ) -> Result<Table, String> {
        let mut table = Table::new(header)?;

        for row in range.rows().skip(1) {
            for (col_idx, cell) in row.iter().enumerate() {
                let header = match header.get(col_idx) {
                    Some(h) => h,
                    None => {
                        return Err(
                            format!("Column {} is missing", col_idx)
                        );
                    },
                };

                let cell_value = match cell {
                    Data::String(s) => s.to_string(),
                    Data::DateTime(d) => d.to_string(),
                    Data::Float(f) => f.to_string(),
                    Data::Int(i) => i.to_string(),
                    _ => String::from("")
                };

                match table.push(header, &cell_value) {
                    Ok(_) => {},
                    Err(e) => {
                        return Err(e);
                    },
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
                },
                None => {
                    return Err("There are no sheets on this workbook".to_string())
                }
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