use std::collections::{HashMap, hash_map::Entry};

pub struct Table {
    table: HashMap<String, Vec<String>>,
}

pub trait TableTrait {
    fn new(headers: &Vec<String>) -> Result<Table, String>;
    fn push(&mut self, key: &str, value: &str) -> Result<(), String>;
    fn push_keys(&mut self, keys: &Vec<String>) -> Result<(), String>;
    fn get(&self, key: &str) -> Result<Table, String>;
    fn rename_key(&mut self, key_mapper: HashMap<String, String>) -> Result<(), String>;
    fn push_all(&mut self, key: &str, values: &[String]) -> Result<(), String>;
    fn group_by(&mut self, keys: &[String]) -> Result<Vec<Table>, String>;
    fn rows(&self) -> Vec<HashMap<&str, &str>>;
    fn get_first(&self, key: &str) -> Result<&str, String>;
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
    
    fn get(&self, key: &str) -> Result<Table, String> {
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
            if old_key == new_key {
                continue;
            }

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
    
    fn group_by(&mut self, keys: &[String]) -> Result<Vec<Table>, String> {
        for key in keys {
            match self.table.contains_key(key) {
                true => {},
                false => {
                    return Err(
                        format!("Key not found: {}", key)
                    );
                },
            }
        }

        let len = match self.table.values().next() {
            Some(v) => v.len(),
            None => {
                return Ok(vec![]);
            },
        };

        for (k, v) in &self.table {
            if v.len() != len {
                return Err(
                    format!("Column '{}' has inconsistent length", k)
                );
            }
        }

        let all_keys: Vec<String> = self.table.keys().cloned().collect();

        let mut groups: HashMap<Vec<String>, Vec<usize>> = HashMap::new();

        for i in 0..len  {
            let group_key = keys
                .iter()
                .map(|k| self.table[k][i].clone())
                .collect();

            groups.entry(group_key).or_default().push(i);
        }

        let mut result = Vec::new();

        for (_group_keys, indices) in groups {
            let mut new_table = Table::new(&all_keys)?;

            for col in &all_keys {
                let values = &self.table[col];

                let selected: Vec<String> = indices
                    .iter()
                    .map(|&i| values[i].clone())
                    .collect();

                new_table.push_all(col, &selected)?;
            }

            result.push(new_table);
        }

        Ok(result)
    }
    
    fn rows(&self) -> Vec<HashMap<&str, &str>> {
        let len = match self.table.values().next() {
            Some(v) => v.len(),
            None => return vec![],
        };

        let mut result: Vec<HashMap<&str, &str>> = Vec::new();

        for i in 0..len {
            let mut row = HashMap::new();

            for (k, v) in &self.table {
                row.insert(k.as_str(), v[i].as_str());
            }

            result.push(row);
        }

        result
    }
    
    fn get_first(&self, key: &str) -> Result<&str, String> {
        match self.table.get(key) {
            Some(v) => {
                match v.first() {
                    Some(vf) => Ok(vf),
                    None => Err(
                        format!("No item found in: {}", key)
                    ),
                }
            },
            None => Err(
                format!("Key not found: {}", key)
            ),
        }
    }
}