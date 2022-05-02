use actix::{Actor, StreamHandler};
use actix_web::http::Error;
use actix_web::web::Bytes;
use actix_web::{
    body, get, http::header, http::StatusCode, post, web, App, HttpRequest, HttpResponse,
    HttpResponseBuilder, HttpServer, Responder, Result,
};
use actix_web_actors::ws;
use futures::future::ok;
use futures::stream::once;
use futures::StreamExt;
use reqwest::header::HeaderValue;
use reqwest::Response;
use reqwest::{self, header as req_header, StatusCode as req_StatusCode};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::fs;
use std::io::prelude::*;
use std::io::{Read, SeekFrom};

pub async fn stream_video(name: web::Path<String>, request: HttpRequest) -> impl Responder {
    let mut file = fs::File::open(std::path::Path::new(&format!(
        "./static/{}",
        name.into_inner()
    )))
    .unwrap();
    let file_size = file.metadata().unwrap().len();

    let range: Vec<String> = request
        .headers()
        .get("range")
        .unwrap()
        .to_str()
        .unwrap()
        .replace("bytes=", "")
        .clone()
        .split("-")
        .map(|el| el.to_string())
        .collect();
    let start = range[0].parse::<u64>().unwrap();
    let end = if range[1] == "".to_string() {
        file_size - 1
    } else {
        range[1].parse::<u64>().unwrap()
    };
    file.seek(SeekFrom::Start(start)).unwrap();
    let mut output = vec![0u8; 20480];
    file.read_exact(&mut output).unwrap_or_else(|_| {
        HttpResponse::Ok()
            .status(StatusCode::from_u16(200).unwrap())
            .append_header(("Content-Length", file_size))
            .append_header(("Content-Type", "video/mp4"))
            .body("ok");
    });

    let stream = once(ok::<_, Error>(Bytes::copy_from_slice(&output)));
    HttpResponse::Ok()
        .status(StatusCode::from_u16(206).unwrap())
        .append_header((
            "Content-Range",
            format!("bytes {}-{}/{}", start, end, file_size),
        ))
        .append_header(("Accept-Ranges", "bytes"))
        .append_header(("Content-Type", "video/mp4"))
        .append_header(("Content-Length", (end - start) + 1))
        .streaming(stream)
}

pub async fn naive_stream_video(req: HttpRequest) -> HttpResponse {
    let file_path = std::path::Path::new("./static/video.mp4");
    let file = actix_files::NamedFile::open_async(file_path).await.unwrap();

    file.into_response(&req)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataFile {
    pub from: u64,
    pub to: u64,
}
pub async fn get_chunk_video(name: web::Path<String>, data: web::Json<DataFile>) -> impl Responder {
    let range: (u64, u64) = (data.0.from, data.0.to);
    let status = request_video_chunk(name.into_inner(), range).await.status();
    HttpResponse::Ok().body("Ok")
}

async fn request_video_chunk(name: String, range: (u64, u64)) -> Response {
    let mut file = fs::File::open(std::path::Path::new(&format!("./static/{}", name))).unwrap();
    let type_file: &str = name.as_str().split(".").collect::<Vec<&str>>()[1];
    let type_for_headers = match type_file {
        "mp4" => "video/mp4",
        "txt" => "text/plain",
        _ => "text/plain",
    };
    let file_size = file.metadata().unwrap().len();
    let start = range.0;
    let end = range.1;
    let _length = if (end - start) > file_size {
        file_size
    } else {
        (end - start)
    };
    let length = format!("{}", _length);
    file.seek(SeekFrom::Start(start)).unwrap();
    let mut output = vec![0u8; _length as usize];
    file.read_exact(&mut output).unwrap();

    let stream = Bytes::copy_from_slice(&output);
    let client = reqwest::Client::new();
    let mut headers = req_header::HeaderMap::new();
    headers.insert(req_header::CONTENT_TYPE, type_for_headers.parse().unwrap());
    headers.insert(req_header::ACCEPT_RANGES, "bytes".parse().unwrap());
    headers.insert(req_header::CONTENT_LENGTH, length.parse().unwrap());
    let mut url = String::from("http://[::1]:3000/get/");
    url.push_str(&name);
    client
        .post(&url)
        .headers(headers)
        .body(reqwest::Body::from(stream))
        .send()
        .await
        .unwrap()
}

pub async fn get_byte_to_server(name: String, range: DataFile) ->  Result<reqwest::Response, reqwest::Error>{
    let mut file = fs::File::open(std::path::Path::new(&format!("./static/{}", name))).unwrap();
    let file_size = file.metadata().unwrap().len();
    let start = range.from;
    let end = range.to;
    let _length = if (end - start) > file_size {
        file_size
    } else {
        end - start
    };
    let length = format!("{}", _length);
    file.seek(SeekFrom::Start(start)).unwrap();
    let mut output = vec![0u8; _length as usize];
    file.read_exact(&mut output).unwrap();

    let stream = Bytes::copy_from_slice(&output);
    let client = reqwest::Client::new();
    let mut headers = req_header::HeaderMap::new();
    headers.insert(req_header::CONTENT_LENGTH, HeaderValue::from_str(&length).unwrap());
    let mut url = String::from("http://[::1]:3000/get/");
    url.push_str(&name);
    
    client
    .post(&url)
    .headers(headers)
    .body(reqwest::Body::from(stream))
    .send()
    .await
}

#[derive(Serialize)]
struct Dir {
    dir: Vec<String>,
}
pub fn get_directory() -> Vec<String> {
    let mut dir = Vec::new();
    for entry in fs::read_dir(std::path::Path::new("./static")).unwrap() {
        dir.push(
            entry
                .unwrap()
                .path()
                .to_str()
                .unwrap()
                .replace("./static\\", ""),
        );
    }
    dir
}
