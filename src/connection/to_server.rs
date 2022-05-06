use websocket;
use std::{fs, thread};
use actix_web::web::{Bytes, BytesMut};
use std::io::{SeekFrom, stdin};
use std::sync::mpsc::channel;
use websocket::client::ClientBuilder;
use websocket::{Message, OwnedMessage};
use serde_json::{json};
use serde::{Serialize, Deserialize};
use futures::executor::block_on;
use tokio;

#[path = "../stream/video.rs"] mod stream;

#[derive(Serialize, Deserialize)]
pub struct DataFile {
	pub from: u64,
	pub to: u64,
}


pub fn connected_to_server_ws(addres: String) {
	let client = ClientBuilder::new(&addres)
		.unwrap()
		.add_protocol("rust-websocket")
		.connect_insecure()
		.unwrap();
	
	let (mut receiver, mut sender) = client.split().unwrap();
	
	let (tx, rx) = channel();
	
	let tx_1 = tx.clone();
	
	
	let receive_loop = thread::spawn(move || {
		// Receive loop
		for message in receiver.incoming_messages() {
			let message = match message {
				Ok(m) => m,
				Err(e) => {
					println!("Receive Loop: {:?}", e);
					let _ = tx_1.send(OwnedMessage::Close(None));
					return;
				}
			};
			match message {
				OwnedMessage::Close(_) => {
					// Got a close message, so send a close message and return
					let _ = tx_1.send(OwnedMessage::Close(None));
					return;
				}
				OwnedMessage::Ping(data) => {
					match tx_1.send(OwnedMessage::Pong(data)) {
						// Send a pong in response
						Ok(()) => (),
						Err(e) => {
							println!("Receive Loop: {:?}", e);
							return;
						}
					}
				}
				OwnedMessage::Text(message_text) => {
                    println!("{}", message_text);
					if message_text.starts_with("get ") {
						let data_json: stream::DataFile = serde_json::from_str(message_text.split("get ").collect::<Vec<&str>>()[1]).unwrap();
                        println!("{:?}", data_json);
                        async_fn_run(data_json);
					}
				}
				// Say what we received
				_ => println!("Receive Loop: {:?}", message),
			}
		}
	});
	
	let _ = receive_loop.join();
	
	println!("Exited");
}

fn async_fn_run(data: stream::DataFile){
    let rt = tokio::runtime::Runtime::new().unwrap();
    let handle = rt.handle();
    let sync_fn = stream::get_byte_to_server(String::from("sample.mp4"), data);
    handle.block_on(sync_fn);
}