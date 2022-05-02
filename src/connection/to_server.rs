use websocket;
use std::{fs, thread};
use actix_web::web::{Bytes, BytesMut};
use std::io::{SeekFrom, stdin};
use std::sync::mpsc::channel;
use reqwest::header::HeaderValue;
use reqwest::Response;
use websocket::client::ClientBuilder;
use websocket::{Message, OwnedMessage};
use serde_json::{json};
use serde::{Serialize, Deserialize};
use futures::executor::block_on;


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
                        block_on(test_send(data_json));
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

fn test_send(data_json: stream::DataFile) -> impl futures::Future<Output = ()>{
    async move{
        stream::get_byte_to_server(String::from("test.txt"), data_json).await.unwrap();
    }
}