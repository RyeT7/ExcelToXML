use std::collections::{HashMap, hash_map::Entry};

pub struct Table {
    pub table: HashMap<String, Vec<String>>,
}

pub trait TableTrait {
    fn new(headers: &Vec<String>) -> Table;
    fn push(&mut self, key: &str, value: String) -> Result<(), String>;
    fn push_keys(&mut self, keys: &Vec<String>) -> Result<(), String>;
}

impl TableTrait for Table {
    fn new(headers: &Vec<String>) -> Table {
        let mut table = Table {
            table: HashMap::new()
        };

        table.push_keys(headers);

        table
    }
    
    fn push(&mut self, key: &str, value: String) -> Result<(), String> {
        match self.table.get_mut(key) {
            Some(v) => {
                v.push(value);
            },
            None => {
                return Err(
                    format!("Key does not exist: {}", key)
                );
            },
        }

        Ok(())
    }
    
    fn push_keys(&mut self, keys: &Vec<String>) -> Result<(), String> {        
        for key in keys {
            match self.table.entry(key.to_string()) {
                Entry::Occupied(o) => {
                    return Err(
                        format!("Duplicate key found: {}", o.key())
                    );
                },
                Entry::Vacant(v) => {
                    v.insert(Vec::new());
                },
            }
        }

        Ok(())
    }
}