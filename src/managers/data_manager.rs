use std::{io::{BufWriter}, fs::File, collections::HashMap};
use serde_json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct EntryBody {
    pub entry: String,
    pub doc: Option<String>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SettingsBody {
    pub password: String,
    pub port: u16,
    pub address: String,
    pub workers: usize,
    pub snapshot_seconds: u64,
    pub entry_limit: usize
}

pub struct DataManager {
    pub db: HashMap<String, serde_json::Value>,
    pub db_path: String
}

impl DataManager {
    pub fn new(db_location: &str) -> Self {
        let db = load_db(db_location);
        let db_path = db_location.to_string();
        DataManager { db, db_path }
    }

    pub fn get(&self, resource: &EntryBody) -> Option<&serde_json::Value> {
        if let Some(doc) = resource.doc.clone() {
            let data = self.db.get(&doc);
            if let Some(res) = data {
                res.get(resource.entry.clone())
            } else {
                None
            }
        } else {
            self.db.get(&resource.entry)
        }
    }

    pub fn extend(&mut self, data: serde_json::Value) {
        for (k, v) in data.as_object().unwrap() {
            self.db.insert(k.to_owned(), v.to_owned());
        }
    }

    pub fn remove(&mut self, resource: &EntryBody) -> Option<serde_json::Value> {
        if let Some(doc) = resource.doc.clone() {
            let data: Option<&mut serde_json::Value> = self.db.get_mut(&doc);
            if let Some(res) = data {
                res.as_object_mut().unwrap().remove(&resource.entry)
            } else {
                None
            }
        } else {
            self.db.remove(&resource.entry)
        }
    }
}

pub fn load_json(path: &str) -> BufWriter<File> {
    let f = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .read(false)
        .open(path)
        .unwrap();
    std::io::BufWriter::new(f)
}

fn load_db(path: &str) -> HashMap<String, serde_json::Value> {
    let db_data = std::fs::read_to_string(path).expect("Failed to read");
    let db_json: serde_json::Value = serde_json::from_str(&db_data).expect("Make sure to type '{}' inside of db.json as well as properly set up settings.json.");
    serde_json::from_value(db_json).unwrap()
}

pub fn load_data(path: &str) -> SettingsBody {
    unsafe {
        let settings_data = std::fs::read_to_string(path).expect("Failed to read");
        let settings_json: SettingsBody = serde_json::from_str(&settings_data).expect("Make sure to type '{}' inside of db.json as well as properly set up settings.json.");
        SETTINGS = Some(settings_json.clone());
        settings_json
    }
}

pub fn write_to_json(value: &HashMap<String, serde_json::Value>, path: &str) {
    let file = load_json(path);
    serde_json::to_writer(file, value).unwrap();
}

lazy_static::lazy_static! { pub static ref MANAGER: parking_lot::Mutex<Option<DataManager>> = parking_lot::Mutex::new(Some(DataManager::new("./dbs/db.json")));}
pub static mut SETTINGS: Option<SettingsBody> = None;
