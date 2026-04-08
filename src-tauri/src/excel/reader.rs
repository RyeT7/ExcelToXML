use std::collections::HashMap;

use calamine::{Data, Range, Reader, open_workbook_auto};

pub struct ExcelReader<'a> {
    path: &'a str,
    range: Range<Data>,
    pub data_row: Vec<HashMap<String, String>>,
    pub header: HashMap<usize, String>,
}

pub trait ExcelReaderTrait<'a> {
    fn new(path: &'a str) -> Result<ExcelReader<'a>, String>;
    fn read_excel(&mut self) -> Result<(), &'static str>;
}

impl<'a> ExcelReader<'a> {
    fn extract_header(range: &Range<Data>) -> Result<HashMap<usize, String>, String> {
        let header_row = match range.rows().into_iter().next() {
            Some(r) => r,
            None => {
                return Err("Header rows not found".to_string())
            }
        };

        let mut headers = HashMap::new();

        for (col_idx, cell) in header_row.iter().enumerate() {
            let cell_value = match cell {
                Data::String(s) => s.to_string(),
                Data::DateTime(d) => d.to_string(),
                Data::Float(f) => f.to_string(),
                Data::Int(i) => i.to_string(),
                _ => String::from("")
            };

            match headers.insert(col_idx, cell_value) {
                Some(v) => {
                    let error_msg = format!("The column {} was referenced more than once", v);
                    return Err(error_msg);
                },
                None => {},
            };
        }

        Ok(headers)
    }
}

impl<'a> ExcelReaderTrait<'a> for ExcelReader<'a> {
    fn new(path: &'a str) -> Result<ExcelReader<'a>, String> {
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

        Ok(ExcelReader {
            path,
            range: range,
            data_row: Vec::new(),
            header: header,
        })
    }
    
    fn read_excel(&mut self) -> Result<(), &'static str> {
        for (_row_idx, row) in self.range.rows().enumerate().skip(1) {
            let mut row_map: HashMap<String, String> = HashMap::new();

            for (col_idx, cell) in row.iter().enumerate() {
                let header = match self.header.get(&col_idx) {
                    Some(h) => h,
                    None => {
                        return Err("A column is missing");
                    },
                };

                let cell_value = match cell {
                    Data::String(s) => s.to_string(),
                    Data::DateTime(d) => d.to_string(),
                    Data::Float(f) => f.to_string(),
                    Data::Int(i) => i.to_string(),
                    _ => String::from("")
                };

                row_map.insert(header.to_string(), cell_value);
            }

            self.data_row.push(row_map);
        }

        Ok(())
    }
}