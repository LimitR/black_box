use std::fs;
use std::io::{self, stdin, stdout, Write};


pub fn start() -> Vec<String>{
	let mut message_user = String::new();
	let mut result = Vec::new();
	let message_app = [
		"Добавьте нужные видео в папку static и нажмите Enter".to_string(),
		"Добавлены все нужные файлы?\n-> Y/N (Yes/No)".to_string(),
	];
	let mut count = 0;
	loop{
		println!("{}", &message_app[count]);
		if count == 1 {
			let mut dir = Vec::new();
			for entry in fs::read_dir(std::path::Path::new("./static")).unwrap() {
				dir.push(entry.unwrap().path().to_str().unwrap().replace("./static\\", ""));
			};
		}
		stdin().read_line(&mut message_user).expect("Произошла ошибка");
		if &message_user.trim() == &String::from(""){
			result.push("default".to_string());
		}else{
			result.push(message_user.trim().to_string())
		}
		if count + 1 == message_app.len(){
			break
		}
		count += 1;
	}
	println!("Started to 127.0.0.1:8000");
	result
}


