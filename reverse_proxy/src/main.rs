use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::thread;
use std::sync::{Arc, Mutex};

type SharedSecret = Arc<Mutex<Option<String>>>;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3000").unwrap();

    let secret_state: SharedSecret = Arc::new(Mutex::new(None));

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let secret_state_clone = Arc::clone(&secret_state);
        thread::spawn(move ||{
            proxy_foward(stream, secret_state_clone);
        });
    }
}

fn proxy_foward(mut stream: TcpStream, secret_state: SharedSecret) {
    let mut buffer = [0; 4096];
    stream.read(&mut buffer).unwrap();
    let request_string = String::from_utf8_lossy(&buffer[..]);
    /*println!(
        "Request: {}", String::from_utf8_lossy(&buffer[..])
    );*/
    let mut method = "N/A".to_string();
    let mut path = "N/A".to_string();

    let main_header = request_string.lines().next().unwrap().to_string();
    let mut parts = main_header.split_whitespace();
    method = parts.next().unwrap().to_string();
    path = parts.next().unwrap().to_string();


    if method == "POST" && path == "/register-secret" {

        let body = request_string.split("\r\n\r\n").nth(1).unwrap_or("").trim_end_matches('\0');

        let mut secret_guard = secret_state.lock().unwrap();
        *secret_guard = Some(body.to_string());

        println!("[REVERSE PROXY]: Received server's key: {:?}", *secret_guard);

        let response = "HTTP/1.1 200 OK \r\n\r\n";
        stream.write(response.as_bytes()).unwrap();

    } else {
        let secred_guard = secret_state.lock().unwrap();
        let secret = match &*secred_guard {
            Some(s) => s.clone(),
            None => {
                let response = "HTTP/1.1 503 Service Unavaible\r\n\r\nProxy not ready";
                stream.write(response.as_bytes()).unwrap();
                return;
            }
        };
        drop(secred_guard);

        if method.starts_with("GET") {
            println!("[REVERSE PROXY]: Recebemos uma requisição do tipo: GET");

            let mut server_stream = TcpStream::connect("127.0.0.1:7878").unwrap();
            let server_req = format!(
                "X-Proxy-Signature: {}\r\n{} {}\r\nHost: 127.0.0.1:7878\r\n\r\n",secret, method, path
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
            println!("[REVERSE PROXY]: Recebemos uma nova requisição não-roteada => \n{}", request_string);
        }
    } 
}
