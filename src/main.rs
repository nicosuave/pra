#![feature(plugin)]
#![plugin(serde_macros)]
#![feature(custom_derive)]
#![feature(custom_attribute)]

extern crate serde;
extern crate serde_json;

use std::env;
use std::fs::File;
use std::io::Read;

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
                    let deserialized: Result<Foo, serde_json::Error> = serde_json::from_str(&config_string);
                    // deserialized.unwrap();
                    match deserialized {
                        Ok(v) => println!("Your ~/.pra.config was read successfully: {:?}", v.foo),
                        Err(_) => println!("Your ~/.pra.config is invalid")
                    }
                },
                Err(_) => println!("Could not read your ~/pra.json")
            }
        }
        None => println!("Could not find your home directory!")
    }
}

// {
//   "pull_sources": [
//     {
//       "type": "github",
//       "config": {
//         "protocol": "https",
//         "host": "api.github.com",
//         "username": "your.username",
//         "password": "your.password",
//         "repositories": [
//           { "owner": "reachlocal", "repository": "snapdragon" },
//           { "owner": "brewster", "repository": "cequel" }
//         ]
//       }
//     }
//   ]
// }

#[derive(Debug)]
#[derive(Deserialize)]
struct Foo {
    foo: String
}

#[derive(Debug)]
#[derive(Deserialize)]
struct PraConfig {
    pull_sources: Vec<PullSource>
}

#[derive(Debug)]
#[derive(Deserialize)]
struct PullSource {
    service: String,
    pull_sources: Vec<Config>
}

#[derive(Debug)]
#[derive(Deserialize)]
struct Config {
    protocol: String,
    host: String,
    username: String,
    password: String,
    repositories: Vec<Repository>
}

#[derive(Debug)]
#[derive(Deserialize)]
struct Repository {
    owner: String,
    repository: String
}
