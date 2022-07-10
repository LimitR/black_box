use std::net::UdpSocket;

pub fn start_stream() {
    let socket = UdpSocket::bind("127.0.0.1:3400").expect("couldn't bind to address");
}