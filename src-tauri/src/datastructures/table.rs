use std::{collections::{HashMap, hash_map::Entry}, fmt::format};

pub struct Table {
    table: HashMap<String, Vec<String>>,
}

pub trait TableTrait {
    fn new(headers: &Vec<String>) -> Result<Table, String>;
    fn push(&mut self, key: &str, value: &str) -> Result<(), String>;
    fn push_keys(&mut self, keys: &Vec<String>) -> Result<(), String>;
    fn get(&mut self, key: &str) -> Result<Table, String>;
    fn rename_key(&mut self, key_mapper: HashMap<String, String>) -> Result<(), String>;
    fn push_all(&mut self, key: &str, values: &[String]) -> Result<(), String>;
    fn remove_duplicates(&mut self) -> Result<(), String>;
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
    
    fn push(&mut self, key: &str, value: &str) -> Result<(), String> {
        match self.table.get_mut(key) {
            Some(v) => {
                v.push(value.to_string());
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
    
    fn get(&mut self, key: &str) -> Result<Table, String> {
        match self.table.get(key) {
            Some(v) => {
                let header: Vec<String> = vec![key.to_string()];

                let mut table = Table::new(&header)?;

                match table.push_all(key, v) {
                    Ok(_) => {},
                    Err(e) => {
                        return Err(e);
                    },
                };

                Ok(table)
            },
            None => Err(
                format!("Key not found: {}", key)
            ),
        }
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
    
    fn push_all(&mut self, key: &str, values: &[String]) -> Result<(), String> {
        match self.table.get_mut(key) {
            Some(v) => {
                v.extend(values.iter().cloned());
            },
            None => {
                return Err(
                    format!("Key does not exist: {}", key)
                );
            },
        }

        Ok(())
    }
    
    fn remove_duplicates(&mut self) -> Result<(), String> {
        Ok(())
    }
}