use serde::{Serialize, Deserialize};
use druid::{Data, Lens};
use std::{fs, thread};

#[path = "../connection/to_server.rs"] mod to_server;

#[derive(Clone, Data, Lens)]
pub struct ForStartApp {
    pub url_api: String,
    pub token: String,
    pub login: String,
    pub password: String,
    pub save: bool,
    pub lable: String,
}


#[derive(Deserialize, Serialize)]
pub struct Config {
    pub url_api: String,
    pub token: String,
    pub login: String,
    pub password: String,
}


impl ForStartApp {

    pub fn is_config() -> Config {
        let file =  fs::read_to_string("./config/user.json")
            .unwrap_or_else(|_| String::from("{ \"url_api\": \"\", \"token\": \"\", \"login\": \"\", \"password\": \"\" }"));
            Config { ..serde_json::from_str(&file).unwrap() }
    }

    pub fn get_config(&self) -> Config {
        Config {
            url_api: self.url_api.clone(),
            token:  self.token.clone(),
            login: self.login.clone(),
            password: self.password.clone(),
        }
    }

    pub fn save_field(&self){
        if self.save {
            fs::write("./config/user.json", serde_json::to_string(&self.get_config()).unwrap()).unwrap();
        }
    }

    pub fn set_lable(&mut self, string: String){
        self.lable = string;
    }

    pub fn connection(&self){
        let addres = self.url_api.clone();
        thread::spawn(|| to_server::connected_to_server_ws(addres));
    }
}