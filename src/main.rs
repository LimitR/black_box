mod stream;
mod connection;

use actix_web::web::Bytes;
use druid::widget::{Button, Flex, Label, TextBox, Checkbox, LabelText};
use druid::{AppLauncher, Data, ExtEventSink, KeyEvent, Lens, LocalizedString, MouseEvent, PlatformError, Widget, WidgetExt, WindowDesc, Color, EventCtx, Event, Env};
use futures_util::task::Spawn;
use futures_util::{FutureExt, SinkExt as _, StreamExt as _};
use reqwest::{self, RequestBuilder};
use serde_json::{json, };
use serde::{Serialize, Deserialize};
use tokio::sync::mpsc::Sender;
use std::collections::HashMap;
use std::fmt::Error;
use std::future::Future;
use std::io::BufRead;
use std::pin::Pin;
use std::{io, thread};
use std::process::Output;
use std::sync::Arc;
use std::fs;


fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder).title("Black box").window_size((760.0, 680.0));
    let conf = ForStartApp::is_config();
    let data = ForStartApp {
        save: false,
        lable: String::from("App off"),
        url_api: conf.url_api,
        token: conf.token,
        login: conf.login,
        password: conf.password,
    };
    AppLauncher::with_window(main_window).launch(data)
}

#[derive(Clone, Data, Lens, Debug, Deserialize, Serialize)]
struct ForStartApp {
    url_api: String,
    token: String,
    login: String,
    password: String,
    save: bool,
    lable: String
}

#[derive(Deserialize, Serialize)]
struct Config {
    url_api: String,
    token: String,
    login: String,
    password: String,
}

impl ForStartApp {

    fn is_config() -> Config {
        let file =  fs::read_to_string("./config/user.json")
            .unwrap_or_else(|_| String::from("{ \"url_api\": \"\", \"token\": \"\", \"login\": \"\", \"password\": \"\" }"));
            Config { ..serde_json::from_str(&file).unwrap() }
    }

    fn get_config(&self) -> Config {
        Config {
            url_api: self.url_api.clone(),
            token:  self.token.clone(),
            login: self.login.clone(),
            password: self.password.clone(),
        }
    }

    fn save_field(&self){
        if self.save {
            fs::write("./config/user.json", serde_json::to_string(&self.get_config()).unwrap()).unwrap();
        }
    }

    fn set_lable(&mut self, string: String){
        self.lable = string;
    }

    fn connection(&self){
        let addres = self.url_api.clone();
        thread::spawn(move || connection::to_server::connected_to_server_ws(addres));
    }
}

fn ui_builder() -> impl Widget<ForStartApp> {
    let label = Label::new(|data: &String, _env: &_| data.clone())
        .padding(5.0)
        .center()
        .lens(ForStartApp::lable);
    let list_video = Label::new(String::from("Video for loading:")).padding(5.0).center();
    let dir_list = Label::new(stream::video::get_directory().join("\n"))
        .padding(5.0);
    let address_api = TextBox::new()
        .with_placeholder("URL to server")
        .fix_width(200.0)
        .padding(2.)
        .lens(ForStartApp::url_api);

    let secret_token = TextBox::new()
        .with_placeholder("Your secret token")
        .fix_width(200.0)
        .padding(2.)
        .lens(ForStartApp::token);


    let login = TextBox::new()
        .with_placeholder("Login")
        .fix_width(200.0)
        .padding(2.)
        .lens(ForStartApp::login);


    let password = TextBox::new()
        .with_placeholder("Password")
        .fix_width(200.0)
        .padding(2.)
        .lens(ForStartApp::password);
    
    
    let button = Button::new("Start")
        .on_click(|ctx, data: &mut ForStartApp, env| {
            let address = String::from(data.url_api.clone());
            data.set_lable(String::from("App load..."));
            data.connection();
            data.set_lable(String::from("App started"));
            data.save_field();
        })
        .padding(5.0)
        .fix_size(160., 60.);
    
    

    let checkbox = Checkbox::new("Save profile")
    .center()
    .lens(ForStartApp::save);

    Flex::column()
        .with_child(label)
        .with_child(button)
        .with_child(checkbox)
        .with_child(address_api)
        .with_child(secret_token)
        .with_child(login)
        .with_child(password)
        .with_child(list_video)
        .with_child(dir_list)
}
