use std::fs;
use serde_json::{to_writer, json, to_writer_pretty};
use num_cpus;

pub fn setup() {
    let db_exists = std::path::Path::new("./dbs").is_dir();
    if !db_exists {
        fs::create_dir("./dbs").expect("Failed to create directory");
        let db = fs::File::create("./dbs/db.json").expect("Failed to create DB file");
        to_writer(db, &json!({})).expect("Failed to write to DB");
        let settings = fs::File::create("./dbs/settings.json").expect("Failed to create settings file");
        to_writer_pretty(settings, &json!({
            "address": "127.0.0.1",
            "port": 8080,
            "password": "",
            "snapshot_seconds": 30,
            "workers": num_cpus::get_physical(),
            "entry_limit": 0
        })).expect("Failed to write to settings");
        println!("WARNING: No settings.json file was found so the password is empty. It is highly recommended you set one.");
    } else {
        let db_file_exists = std::path::Path::new("./dbs/db.json").exists();
        if !db_file_exists {
            let db = fs::File::create("./dbs/db.json").expect("Failed to create DB file");
            to_writer(db, &json!({})).expect("Failed to write to DB");
        }
        let settings_file_exists = std::path::Path::new("./dbs/settings.json").exists();
        if !settings_file_exists {
            let settings = fs::File::create("./dbs/settings.json").expect("Failed to create settings file");
            to_writer_pretty(settings, &json!({
            "address": "127.0.0.1",
            "port": 8080,
            "password": "",
            "snapshot_seconds": 30,
            "workers": num_cpus::get_physical(),
            "entry_limit": 0
            })).expect("Failed to write to settings");
            println!("WARNING: No settings.json file was found so the password is empty. It is highly recommended you set one.");
        }
    }
}