use actix_web::web::Bytes;
use futures::Future;
use futures::future::ok;
use futures::stream::once;
use reqwest::header::HeaderValue;
use reqwest::Response;
use reqwest::{self, header as req_header, StatusCode as req_StatusCode};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::prelude::*;
use std::io::{Read, SeekFrom};


#[derive(Serialize, Deserialize, Debug)]
pub struct DataFile {
    pub from: u64,
    pub to: u64,
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

pub fn get_byte_to_server(name: String, range: DataFile) -> impl Future<Output = Result<reqwest::Response, reqwest::Error>>  {
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
