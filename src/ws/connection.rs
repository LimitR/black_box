use actix_web_actors::ws::{self};
use actix::{Actor, ActorContext, AsyncContext, StreamHandler};
use actix_web::{
	body, get, http::header, http::StatusCode, post, web, App, HttpRequest, HttpResponse,
	HttpResponseBuilder, HttpServer, Responder, Result,
};
use bytestring::ByteString;
use local_ip_address::local_ip;
struct MyWs;

impl Actor for MyWs {
	type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
	fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
		let ip = local_ip().unwrap();
		match msg {
			Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
			Ok(ws::Message::Text(text)) => {
				ctx.text(ByteString::from(String::from(ip.to_string())))
			},
			Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
			Ok(ws::Message::Close(reason)) => {
				ctx.close(reason);
				ctx.stop();
			},
			_ => (),
		}
	}
}


impl MyWs {
	fn send_message_to_user(&mut self) {
		ws::Message::Text(ByteString::from(String::from("Hello")));
	}
}

pub async fn get_message(req: HttpRequest, stream: web::Payload) -> impl Responder {
	// client.handle(ws::Message::Text(ByteString::from(String::from("hello"))));
}

pub async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, actix_web::Error> {
	let resp = ws::start(MyWs {}, &req, stream);
	println!("{:?}", resp);
	resp
}
