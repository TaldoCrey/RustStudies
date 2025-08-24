use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        proxy_foward(stream);
    }
}

fn proxy_foward(mut stream: TcpStream) {
    let mut buffer = [0; 4096];
    stream.read(&mut buffer).unwrap();

    println!(
        "Request: {}", String::from_utf8_lossy(&buffer[..])
    );

    let get = b"GET /";

    if buffer.starts_with(get) {
        println!("[REVERSE PROXY]: Recebemos uma requisição do tipo: GET");

        let mut server_stream = TcpStream::connect("127.0.0.1:7878").unwrap();
        server_stream.write_all(&buffer).unwrap();
        server_stream.flush().unwrap();

        println!("[REVERSE PROXY]: Fowarded requisition!");
        let mut response_buffer = [0; 4096];
        server_stream.read(&mut response_buffer).unwrap();
        println!(
            "Response: {}", String::from_utf8_lossy(&response_buffer[..])
        );
        stream.write(&response_buffer).unwrap();
        stream.flush().unwrap();
        

    }
}
