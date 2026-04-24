use std::collections::HashMap;

use calamine::{Data, Range, Reader, open_workbook_auto};

use crate::domain::datastructures::table::{Table};

pub struct ExcelReader {
    path: String,
    range: Range<Data>,
    pub table: Table,
    header: Vec<String>,
}

pub trait ExcelReaderTrait {
    fn new(path: &str) -> Result<ExcelReader, String>;
    fn read_excel(&mut self) -> Result<(), String>;
    fn get_header(&self) -> &Vec<String>;
}

impl ExcelReader {
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
}

impl ExcelReaderTrait for ExcelReader {
    fn new(path: &str) -> Result<ExcelReader, String> {
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

        let header = match ExcelReader::extract_header(&range) {
            Ok(h) => h,
            Err(e) => {
                return Err(e);
            },
        };

        let table = match Table::new(&header) {
            Ok(t) => t,
            Err(e) => {
                return Err(e);
            },
        };

        Ok(ExcelReader {
            path: path.to_string(),
            range: range,
            table: table,
            header: header,
        })
    }
    
    fn read_excel(&mut self) -> Result<(), String> {
        for row in self.range.rows().skip(1) {
            for (col_idx, cell) in row.iter().enumerate() {
                let header = match self.header.get(col_idx) {
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

                match self.table.push(header, &cell_value) {
                    Ok(_) => {},
                    Err(e) => {
                        return Err(e);
                    },
                };
            }
        }

        Ok(())
    }
    
    fn get_header(&self) -> &Vec<String> {
        &self.header
    }
}