extern crate serde;
extern crate serde_json;
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::fs;


#[derive(Serialize, Deserialize, Debug)]
pub struct SaveProfile {
    pub url: String,
    pub token: String,
    pub login: String,
    pub password: String,
}

impl Clone for SaveProfile {
    fn clone(&self) -> Self {
        SaveProfile {
            url: self.url.clone(),
            token: self.token.clone(),
            login: self.login.clone(),
            password: self.password.clone(),
        }
    }
}

impl SaveProfile {
    pub fn new() -> SaveProfile {
        let conn = Connection::open_in_memory().unwrap();
        SaveProfile {
            url: String::new(),
            token: String::new(),
            login: String::new(),
            password: String::new(),
        }
    }
    pub fn save(self) -> std::io::Result<()> {
        fs::write("./config.json", serde_json::to_string(&self).unwrap())
    }
    pub fn get_profile(self) -> SaveProfile {
        self.clone()
    }
    pub fn update(mut self, data: SaveProfile) {
        self = SaveProfile {
            url: data.url,
            token: data.token,
            login: data.login,
            password: data.password,
        };
    }
    fn remove(mut self) {
        let mut old_data = fs::read_to_string("./config.json").unwrap();
    }
}
