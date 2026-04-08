use std::{collections::HashMap, fs::File, io::BufReader};

use calamine::{Reader, Sheets, open_workbook_auto};

use crate::excel::readerResponse::{ExcelReaderResponse, ExcelReaderResponseTrait};

pub struct ExcelReader<'a> {
    path: &'a str,
    workbook: Sheets<BufReader<File>>,
    data_row: Vec<HashMap<&'a str, &'a str>>,
}

pub trait ExcelReaderTrait<'a> {
    fn new(path: &'a str) -> Result<ExcelReader<'a>, &'static str>;
    fn read_excel(&mut self, sheet_names: Vec<&str>) -> ExcelReaderResponse;
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
    
    fn read_excel(&mut self, sheet_names: Vec<&str>) -> ExcelReaderResponse {
        let mut response = ExcelReaderResponse::new();
        
        for sheet in sheet_names {
            let range = self.workbook.worksheet_range(sheet);

            if let Err(_) = range {
                response.add_error(
                    format!("{} does not exist within the workbook", sheet)
                );
            }
        }

        response
    }
}