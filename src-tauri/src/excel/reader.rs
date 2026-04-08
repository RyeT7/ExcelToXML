use std::{collections::HashMap, fs::File, io::BufReader};

use calamine::{Data, Reader, Sheets, open_workbook_auto};

pub struct ExcelReader<'a> {
    path: &'a str,
    workbook: Sheets<BufReader<File>>,
    pub data_row: Vec<HashMap<String, String>>,
}

pub trait ExcelReaderTrait<'a> {
    fn new(path: &'a str) -> Result<ExcelReader<'a>, &'static str>;
    fn read_excel(&mut self) -> Result<(), &'static str>;
}

impl<'a> ExcelReaderTrait<'a> for ExcelReader<'a> {
    fn new(path: &'a str) -> Result<ExcelReader<'a>, &'static str> {
        let workbook = open_workbook_auto(path);

        if let Ok(w) = workbook {
            Ok(ExcelReader { 
                path: path,
                workbook: w,
                data_row: Vec::new()
            })
        } else {
            Err("Workbook does not exist")
        }

    }
    
    fn read_excel(&mut self) -> Result<(), &'static str> {
        let range = match self.workbook.worksheet_range_at(0) {
            Some(Ok(range)) => range,
            Some(Err(_)) => {
                return Err("Something went wrong: sheet not found in workbook")
            },
            None => {
                return Err("There are no sheets on this workbook")
            }
        };

        for row in range.rows() {
            let mut row_map: HashMap<String, String> = HashMap::new();

            for (col_idx, cell) in row.iter().enumerate() {
                let col_name = format!("Column{}", col_idx);

                let cell_value = match cell {
                    Data::String(s) => s.to_string(),
                    Data::DateTime(d) => d.to_string(),
                    Data::Float(f) => f.to_string(),
                    Data::Int(i) => i.to_string(),
                    _ => String::from("")
                };

                row_map.insert(col_name, cell_value);
            }

            self.data_row.push(row_map);
        }

        Ok(())
    }
}