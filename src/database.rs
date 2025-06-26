use std::{
    fs::{self, File},
    io::Write,
    process::exit,
};

use crate::get_database_path;
pub fn return_raw_db_file(append: bool) -> Option<File> {
    let mut options = File::options();
    options.append(append);
    options.write(true);
    options.read(true);
    let file = match options.open(get_database_path()) {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Cannot read db file");
            return None;
        }
    };
    Some(file)
}
pub fn write_to_db(key: String, new_value: String) {
    let db_contents = get_db(get_database_path());
    let mut index = 0;
    let split = &db_contents.split("\n").collect::<Vec<&str>>();
    let mut exists = false;
    for item in split {
        let key_l: &str = match item.split("=").collect::<Vec<&str>>().get(0).map(|v| &**v) {
            Some(v) => v,
            None => {
                eprintln!("Cannot read key from db contents");
                return;
            }
        };
        if key_l == key {
            // TODO improve efficiency
            // This does not feel especially efficient with clone(), should I use an Rc or alike?
            let mut new = split.clone();
            let formatted = &format!("{}={}", &key, &new_value);
            new[index] = formatted;
            let mut final_string = String::new();
            for value in new {
                final_string.push_str(&format!("{value}\n"));
            }
            let mut options = File::options();
            options.append(false);
            options.write(true);
            let mut file = match options.open(get_database_path()) {
                Ok(v) => v,
                Err(_) => {
                    eprintln!("Cannot read db file");
                    return;
                }
            };
            file.write(final_string.as_bytes()).unwrap();
            exists = true;
        }
        index += 1;
    }
    if !exists {
        return_raw_db_file(true).inspect(|mut v| {
            v.write(format!("{}={}", key, new_value).as_bytes())
                .unwrap();
        });
    }
}
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
pub fn get_value_from_key(key: &String, db_path: String) -> Option<String> {
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
        if key_l == &key {
            return Some(value_l.to_string());
        }
    }
    return None;
}
