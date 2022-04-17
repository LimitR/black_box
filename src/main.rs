mod stream;
mod start;
mod ws;


use actix_web;
use actix_web::{ web, App, HttpServer, http };
use reqwest;
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // start::start::start();
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default()
                      .allowed_origin("http://localhost:3000")
                      .allowed_methods(vec!["GET", "POST"])
                      .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                      .allowed_header(http::header::CONTENT_TYPE)
                      .supports_credentials()
                      .max_age(3600))
            .route("/video/{name}", web::get().to(stream::video::stream_video))
            .route("/test", web::get().to(stream::video::naive_stream_video))
            .route("/ws/", web::get().to(ws::connection::index))
            .route("/dir", web::get().to(stream::video::get_directory))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
