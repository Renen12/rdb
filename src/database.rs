use std::{
    fs::{self, File},
    process::exit,
};
pub fn get_db(path: String) -> String {
    let database_contents = match fs::read_to_string(&path) {
        Ok(v) => v,
        Err(_) => {
            match File::create(&path).inspect(|_| println!("Creating new database...")) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Critical: Cannot create database, {}", e);
                    exit(1);
                }
            };
            String::new()
        }
    };
    database_contents
}
pub fn get_value_from_key(key: String, db_path: String) -> Option<String> {
    let database = get_db(db_path);
    for key_and_value in database.split("\n") {
        let key_and_value: Vec<&str> = key_and_value.split("=").collect();
        let key_l = match key_and_value.get(0) {
            Some(v) => v,
            None => {
                return None;
            }
        };
        let value_l = match key_and_value.get(1) {
            Some(v) => v,
            None => {
                return None;
            }
        };
        if key_l.to_string() == key {
            return Some(value_l.to_string());
        }
    }
    return None;
}
