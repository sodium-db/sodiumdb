use std::sync::Mutex;
use serde_json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct EntryBody {
    pub entry: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SettingsBody {
    pub password: String,
    pub port: u16,
    pub address: String
}


pub struct DataManager {
    pub db: serde_json::Value,
    pub db_path: String
}

impl DataManager {
    pub fn new(db_location: &str, settings_location: &str) -> Self {
        let db = load_db(db_location);
        load_data(settings_location);
        let db_path = db_location.to_string();
        DataManager { db, db_path }
    }

    pub fn get(&self, resource: &str) -> Option<&serde_json::Value> {
        self.db.get(resource)
    }

    pub fn extend(&mut self, data: serde_json::Value) {
        self.db.as_object_mut().unwrap().extend(data.as_object().unwrap().clone());
        let f = load_json(&self.db_path);
        f.set_len(0).unwrap();
        serde_json::to_writer(f, &self.db).unwrap();
    }

    pub fn remove(&mut self, resource: &str) {
        self.db.as_object_mut().unwrap().remove(resource).unwrap();
        let f = load_json(&self.db_path);
        f.set_len(0).unwrap();
        serde_json::to_writer(f, &self.db).unwrap();
    }
}

fn load_json(path: &str) -> std::fs::File {
    std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .read(false)
        .open(path.clone())
        .unwrap()
}

fn load_db(path: &str) -> serde_json::Value {
    let settings_data = std::fs::read_to_string(path).expect("Failed to read");
        let settings_json: serde_json::Value = serde_json::from_str(&settings_data).expect("Make sure to type '{}' inside of db.json as well as properly set up settings.json.");
        settings_json
}

fn load_data(path: &str) -> SettingsBody {
    unsafe {
        let settings_data = std::fs::read_to_string(path).expect("Failed to read");
        let settings_json: SettingsBody = serde_json::from_str(&settings_data).expect("Make sure to type '{}' inside of db.json as well as properly set up settings.json.");
        SETTINGS = Some(settings_json.clone());
        settings_json
    }
}

pub static mut MANAGER: Mutex<Option<DataManager>> = Mutex::new(None);
pub static mut SETTINGS: Option<SettingsBody> = None;