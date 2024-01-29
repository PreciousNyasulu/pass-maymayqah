use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Entries {
    entries: HashMap<String, HashMap<String, String>>,
}

impl Entries {
    pub fn get_entry(&self, entry_name: &str) -> Option<&HashMap<String, String>> {
        self.entries.get(entry_name)
    }

    pub fn get_value(&self, entry_name: &str, key: &str) -> Option<&String> {
        if let Some(entry) = self.get_entry(entry_name) {
            entry.get(key)
        } else {
            None
        }
    }
}

pub fn load_entries(file_path: &str) -> io::Result<Entries> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let entries: Entries = serde_json::from_str(&contents)?;
    Ok(entries)
}