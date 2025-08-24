use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::thread;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(move ||{
            proxy_foward(stream);
        });
    }
}

fn proxy_foward(mut stream: TcpStream) {
    let mut buffer = [0; 4096];
    stream.read(&mut buffer).unwrap();
    let request_string = String::from_utf8_lossy(&buffer[..]);
    /*println!(
        "Request: {}", String::from_utf8_lossy(&buffer[..])
    );*/
    let mut method = "N/A".to_string();
    let mut host = "N/A".to_string();

    for i in 0..2 {
    
        if i == 0 {
            method = request_string.lines().next().unwrap().to_string();
        } else if i == 1 {
            host = request_string.lines().skip(1).next().unwrap().to_string();
        }
    }

    if method.starts_with("GET") {
        println!("[REVERSE PROXY]: Recebemos uma requisição do tipo: GET");

        let mut server_stream = TcpStream::connect("127.0.0.1:7878").unwrap();
        let server_req = format!(
            "{}\r\nHost:{}\r\nKey:revproxy\r\n\r\n", method, host
        );
        server_stream.write_all(server_req.as_bytes()).unwrap();
        server_stream.flush().unwrap();

        println!("[REVERSE PROXY]: Fowarded requisition!");
        let mut response_buffer = [0; 4096];
        server_stream.read(&mut response_buffer).unwrap();
        println!(
            "Response: {}", String::from_utf8_lossy(&response_buffer[..])
        );
        stream.write(&response_buffer).unwrap();
        stream.flush().unwrap();
        

    } else {
        println!("[REVERSE PROXY]: Recebemos uma nova requisição => \n{}", request_string);
    }
}
