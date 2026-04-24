use calamine::{Data, Range};

use crate::domain::datastructures::table::Table;

struct ExcelReaderData {
    path: String,
    range: Range<Data>,
    table: Table,
    header: Vec<String>,
}

impl ExcelReaderData {
    pub fn new(
        path: String,
        range: Range<Data>,
        table: Table,
        header: Vec<String>
    ) -> ExcelReaderData {
        ExcelReaderData {
            path:   path,
            range:  range,
            table:  table,
            header: header,
        }
    }

    pub fn get_table(&self) -> &Table {
        &self.table
    }

    pub fn get_header(&self) -> &Vec<String> {
        &self.header
    }
}