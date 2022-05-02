mod start;
mod stream;
mod connection;

use actix_web::web::Bytes;
use druid::widget::{Button, Flex, Label, TextBox, Checkbox, ClipBox};
use druid::{AppLauncher, Data, ExtEventSink, KeyEvent, Lens, LocalizedString, MouseEvent, PlatformError, Widget, WidgetExt, WindowDesc};
use futures_util::task::Spawn;
use futures_util::{FutureExt, SinkExt as _, StreamExt as _};
use reqwest::{self, RequestBuilder};
use serde_json::{json, };
use serde::{Serialize, Deserialize};
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
    let main_window = WindowDesc::new(ui_builder).title("Black box");
    let data = ForStartApp::new();
    AppLauncher::with_window(main_window).launch(data)
}

#[derive(Clone, Data, Lens, Debug, Deserialize, Serialize)]
struct ForStartApp {
    data: String,
    token: String,
    login: String,
    password: String,
    save: bool
}

impl ForStartApp {

    fn new() -> ForStartApp {
        let file =  fs::read_to_string("./config/user.json")
            .unwrap_or_else(|_| String::from("{ \"data\": \"\", \"token\": \"\", \"login\": \"\", \"password\": \"\", \"save\":\"\" }"));
        ForStartApp { ..serde_json::from_str(&file).unwrap() }
    }

    fn save_field(&self){
        if self.save {
            fs::write("./config/user.json", serde_json::to_string(&self).unwrap()).unwrap();
        }
    }
}

fn ui_builder() -> impl Widget<ForStartApp> {
    let label = Label::new(String::from("Start app")).padding(5.0).center();
    let dir_list = Label::new(stream::video::get_directory().join("\n"))
        .padding(5.0);
    let address_api = TextBox::new()
        .with_placeholder("URL to server")
        .fix_width(200.0)
        .padding(2.)
        .lens(ForStartApp::data);

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
        .on_click(|ctx, data: &mut ForStartApp, _| {
            let addres = String::from(data.data.clone()) ;
            thread::spawn(move||  connection::to_server::connected_to_server_ws(addres));
            data.save_field();
        })
        .padding(5.0)
        .fix_size(160., 60.);
    
    

    let checkbox = Checkbox::new("Save profile")
    .center()
    .lens(ForStartApp::save);

    Flex::column()
        .with_child(label)
        .with_child(dir_list)
        .with_child(button)
        .with_child(checkbox)
        .with_child(address_api)
        .with_child(secret_token)
        .with_child(login)
        .with_child(password)
}
