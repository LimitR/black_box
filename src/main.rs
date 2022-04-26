mod start;
mod stream;
mod ws;

use actix_cors::Cors;
use actix_web;
use actix_web::{connect, http, web, App, HttpServer};
use reqwest::{self, RequestBuilder};
use std::collections::HashMap;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // start::start::start();
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://127.0.0.1:8000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .route("/video/{name}", web::get().to(stream::video::stream_video))
            .route("/native", web::get().to(stream::video::naive_stream_video))
            .route("/ws/", web::get().to(ws::connection::index))
            .route("/dir", web::get().to(stream::video::get_directory))
            // .route("/send", web::get().to(ws::connection::get_message))
            .route(
                "/get/video/{name}",
                web::post().to(stream::video::get_chunk_video),
            )
            .route(
                "/get/byte/{name}",
                web::post().to(stream::video::get_byte),
            )
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
