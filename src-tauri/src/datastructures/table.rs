use std::collections::{HashMap, hash_map::Entry};

pub struct Table {
    table: HashMap<String, Vec<String>>,
}

pub trait TableTrait {
    fn new(headers: &Vec<String>) -> Result<Table, String>;
    fn push(&mut self, key: &str, value: String) -> Result<(), String>;
    fn push_keys(&mut self, keys: &Vec<String>) -> Result<(), String>;
    fn get(&mut self, key: String) -> Result<Vec<String>, String>;
    fn rename_key(&mut self, key_mapper: HashMap<String, String>) -> Result<(), String>;
}

impl TableTrait for Table {
    fn new(headers: &Vec<String>) -> Result<Table, String> {
        let mut table = Table {
            table: HashMap::new()
        };

        match table.push_keys(headers) {
            Ok(_) => {
                return Ok(table);
            },
            Err(e) => {
                return Err(e);
            },
        };
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
    
    fn get(&mut self, key: String) -> Result<Vec<String>, String> {
        todo!()
    }
    
    fn rename_key(&mut self, key_mapper: HashMap<String, String>) -> Result<(), String> {
        for new_key in key_mapper.values() {
            if self.table.contains_key(new_key) {
                return Err(
                    format!("Duplicate key: {}", new_key)
                );
            }
        }

        for (old_key, new_key) in key_mapper {
            match self.table.remove(&old_key) {
                Some(v) => {
                    self.table.insert(new_key, v);
                },
                None => {},
            }
        }

        Ok(())
    }

    
}