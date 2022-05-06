mod stream;
mod connection;
mod ui;
use ui::struct_ui::*;
use druid::{Color, Data, RenderContext, FileSpec};
use druid::widget::{Button, Flex, Label, TextBox, Checkbox, Either, Switch, Painter};
use druid::{AppLauncher, PlatformError, Widget, WidgetExt, WindowDesc, FileDialogOptions};

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
    // ui::test_ui::start();
    // Ok(())
}


fn ui_builder() -> impl Widget<ForStartApp> {
    let label = Either::new(|data: &ForStartApp, _env| 
        data.save, 
        Label::new("App off").padding(5.0).center(),
        Label::new("App start").padding(5.0).center()
    );
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
        .on_click(move |ctx, data: &mut ForStartApp, env| {
            data.set_lable(String::from("App load..."));
            data.connection();
            data.set_lable(String::from("App started"));
            data.save_field();
        })
        .padding(5.0)
        .fix_size(160., 60.);
    

    let checkbox = Checkbox::new("Save profile")
        .padding(5.0).lens(ForStartApp::save);

    
    Flex::column().with_child(label)
        .with_child(button)
        .with_flex_child(checkbox, 1.0)
        .with_child(address_api)
        .with_child(secret_token)
        .with_child(login)
        .with_child(password)
        .with_child(list_video)
        .with_child(dir_list)
}


fn widget_connect_url() -> impl Widget<ForStartApp> {
    let mut layout = Flex::column();
    for _ in 0..10 {
        let address_api = TextBox::new()
        .with_placeholder("URL to server")
        .fix_width(200.0)
        .padding(2.)
        .lens(ForStartApp::url_api);
        let add_new_url_connect = Button::from_label(Label::new(String::from("+")).with_text_color(Color::grey(0.5)));
        let test_btn = Button::new("Connect");
        let button_connect = Flex::row().with_child(add_new_url_connect).with_child(address_api).with_child(test_btn);
        layout.add_flex_child(button_connect, 1.0);
    }
    layout
}