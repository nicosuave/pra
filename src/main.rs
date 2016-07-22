extern crate serde;
extern crate serde_json;
extern crate regex;

use std::fs::File;
use std::io::Read;
use serde_json::Value;
use serde_json::Error;
use std::env;

fn main() {
    read_config();
}

fn read_config() {
    match env::home_dir() {
        Some(path) => {
            match File::open(format!("{}/.pra.json", path.display())) {
                Ok(mut v) => {
                    let mut config_string = String::new();
                    &v.read_to_string(&mut config_string).unwrap();
                    let deserialized: Result<Value, Error> = serde_json::from_str(&config_string);
                    match deserialized {
                        Ok(v) => println!("Your ~/.pra.config was read successfully: {}", v),
                        Err(_) => println!("Your ~/.pra.config is invalid")
                    }
                },
                Err(_) => println!("Could not read your ~/pra.json")
            }
        }
        None => println!("Could not find your home directory!")
    }
}
