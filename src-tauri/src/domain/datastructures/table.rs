use std::collections::{HashMap, hash_map::Entry};

use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Table {
    pub headers: Vec<String>,
    pub data: HashMap<String, Vec<String>>,
}

impl Table {
    pub fn new(headers: &Vec<String>) -> Result<Table, String> {
        let mut table = Table {
            headers: headers.to_vec(),
            data: HashMap::new()
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

    pub fn headers(&self) -> &Vec<String> {
        &self.headers
    }
    
    pub fn push(&mut self, key: &str, value: &str) -> Result<(), String> {
        match self.data.get_mut(key) {
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
    
    pub fn push_keys(&mut self, keys: &Vec<String>) -> Result<(), String> {        
        for key in keys {
            match self.data.entry(key.to_string()) {
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
    
    pub fn get(&self, key: &str) -> Result<Table, String> {
        match self.data.get(key) {
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
    
    pub fn rename_key(&mut self, key_mapper: HashMap<String, String>) -> Result<(), String> {
        for new_key in key_mapper.values() {
            if self.data.contains_key(new_key) {
                return Err(
                    format!("Duplicate key: {}", new_key)
                );
            }
        }

        for (old_key, new_key) in key_mapper {
            if old_key == new_key {
                continue;
            }

            match self.data.remove(&old_key) {
                Some(v) => {
                    self.data.insert(new_key, v);
                },
                None => {},
            }
        }

        Ok(())
    }
    
    pub fn push_all(&mut self, key: &str, values: &[String]) -> Result<(), String> {
        match self.data.get_mut(key) {
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
    
    pub fn group_by(&mut self, keys: &[String]) -> Result<Vec<Table>, String> {
        for key in keys {
            match self.data.contains_key(key) {
                true => {},
                false => {
                    return Err(
                        format!("Key not found: {}", key)
                    );
                },
            }
        }

        let len = match self.data.values().next() {
            Some(v) => v.len(),
            None => {
                return Ok(vec![]);
            },
        };

        for (k, v) in &self.data {
            if v.len() != len {
                return Err(
                    format!("Column '{}' has inconsistent length", k)
                );
            }
        }

        let all_keys: Vec<String> = self.headers.clone();

        // Rows of each group, in the order the groups are first met — which is
        // the order the sheet lists them in. A HashMap has no order of its own
        // and Rust seeds its hasher afresh for every process, so collecting the
        // groups in it alone would emit the invoices in an order that changed
        // on each run of the program.
        let mut groups: Vec<Vec<usize>> = Vec::new();

        // Where each group sits in `groups`. Keys borrow the cells rather than
        // copying them, and a position is only ever inserted once per group,
        // so no row costs a clone.
        let mut position: HashMap<Vec<&str>, usize> = HashMap::new();

        for i in 0..len  {
            let group_key: Vec<&str> = keys
                .iter()
                .map(|k| self.data[k][i].as_str())
                .collect();

            match position.get(&group_key) {
                Some(&group) => groups[group].push(i),
                None => {
                    position.insert(group_key, groups.len());
                    groups.push(vec![i]);
                },
            }
        }

        let mut result = Vec::new();

        for indices in &groups {
            let mut new_table = Table::new(&all_keys)?;

            for col in &all_keys {
                let values = &self.data[col];

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
    
    pub fn rows(&self) -> Vec<HashMap<&str, &str>> {
        let len = match self.data.values().next() {
            Some(v) => v.len(),
            None => return vec![],
        };

        let mut result: Vec<HashMap<&str, &str>> = Vec::new();

        for i in 0..len {
            let mut row = HashMap::new();

            for (k, v) in &self.data {
                row.insert(k.as_str(), v[i].as_str());
            }

            result.push(row);
        }

        result
    }
    
    pub fn get_first(&self, key: &str) -> Result<&str, String> {
        match self.data.get(key) {
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
    
    pub fn column(&self, key: &str) -> Result<&Vec<String>, String> {
        match self.data.get(key) {
            Some(v) => {
                Ok(v)
            },
            None => Err(
                format!("No item found in: {}", key)
            ),
        }
    }

}