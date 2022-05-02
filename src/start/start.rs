use druid::widget::{Button, Flex, Label, TextBox};
use druid::{
    AppLauncher, Data, ExtEventSink, Lens, LocalizedString, PlatformError, Widget, WidgetExt,
    WindowDesc,
};
use futures_util::task::Spawn;
use std::future::Future;
use std::io::BufRead;
use std::pin::Pin;
use std::thread;

#[path = "../stream/video.rs"]
mod stream;

pub fn start() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder);
    let data = ForStartApp {
        data: "".to_string(),
    };
    AppLauncher::with_window(main_window).launch(data)
}
#[derive(Clone, Data, Lens)]
struct ForStartApp {
    data: String,
}

impl ForStartApp {
    fn start_connection_to_server(&self) {

    }
}

fn ui_builder() -> impl Widget<ForStartApp> {
    // The label text will be computed dynamically based on the current locale and count
    let label = Label::new(String::from("Start app")).padding(5.0).center();
    let dir_list = Label::new(stream::get_directory().join("\n"))
        .padding(5.0)
        .center();
    let address_api = TextBox::new()
        .with_placeholder("URL to server")
        .fix_width(200.0)
        .lens(ForStartApp::data);
    let button = Button::new("Start")
        .on_click(|_, data: &mut ForStartApp, _| {
            data.start_connection_to_server();
        })
        .padding(5.0);

    Flex::column()
        .with_child(label)
        .with_child(dir_list)
        .with_child(button)
        .with_child(address_api)
}
