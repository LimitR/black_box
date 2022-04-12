use std::fmt::Debug;
use serde::{Serialize, Deserialize};
use actix_web;
use actix_web::web::{Bytes, Json};
use actix_web::{
    get, post, web, App, HttpRequest, HttpResponse, HttpResponseBuilder, HttpServer,
    Responder, Result, body, http::header, http::StatusCode
};
use std::fs;
use std::io::{BufRead, Read, SeekFrom, Write};
use actix_web::http::Error;
use futures::future::ok;
use futures::stream::{once};
use actix_files;
use std::io::BufReader;
use std::io;
use std::io::prelude::*;

#[derive(Serialize)]
struct FileName {
    files: Vec<String>
}



async fn get_file(request: HttpRequest) -> impl Responder {
    let mut file = fs::File::open(std::path::Path::new("./static/sample.mp4")).unwrap();
    let file_size = file.metadata().unwrap().len();
    let result = match request.headers().get("range") {
        Some(_) => {
            let range: Vec<String> = request.headers()
                .get("range").unwrap().to_str().unwrap().replace("bytes=", "").clone().split("-").map(|el| el.to_string()).collect();
            let start = range[0].parse::<u64>().unwrap();
            let end = if range[1] == "".to_string() {
                file_size - 1
            } else {
                range[1].parse::<u64>().unwrap()
            };
            file.seek(SeekFrom::Start(start)).unwrap();
            let mut output = vec![0u8; 20480];
            file.read_exact(&mut output).unwrap_or_else(|_|{
                 HttpResponse::Ok()
                    .status(StatusCode::from_u16(200).unwrap())
                    .append_header(("Content-Length", file_size))
                    .append_header(("Content-Type", "video/mp4")).body("ok");
            });
    
            let stream = once(ok::<_, Error>(Bytes::copy_from_slice(&output)));
            HttpResponse::Ok()
                .status(StatusCode::from_u16(206).unwrap())
                .append_header(("Content-Range", format!("bytes {}-{}/{}", start, end, file_size)))
                .append_header(("Accept-Ranges", "bytes"))
                .append_header(("Content-Type", "video/mp4"))
                .append_header(("Content-Length", (end - start) + 1))
                .streaming(stream)
        },
        None => {
            file.seek(SeekFrom::Start(0)).unwrap();
            let mut output = vec![0u8; 20480];
            file.read_exact(&mut output).unwrap();
    
            let stream = once(ok::<_, Error>(Bytes::copy_from_slice(&output)));
            HttpResponse::Ok()
                .status(StatusCode::from_u16(200).unwrap())
                .append_header(("Content-Length", file_size))
                .append_header(("Content-Type", "video/mp4"))
                .streaming(stream)
        }
    };
    result
}

async fn test(req: HttpRequest) -> HttpResponse {
    let file_path = std::path::Path::new("./static/video.mp4");
    let file = actix_files::NamedFile::open_async(file_path).await.unwrap();
    
    file.into_response(&req)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .route("/video", web::get().to(get_file))
            .route("/test", web::get().to(test))
    })
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}