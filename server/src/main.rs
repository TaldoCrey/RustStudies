use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::thread;
use std::path::Path;


fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 4096];
    stream.read(&mut buffer).unwrap();
    
    let request_string = String::from_utf8_lossy(&buffer[..]);

    let mut method = "N/A".to_string();
    let mut value = "N/A".to_string();
    let mut host = "N/A".to_string();
    let mut key = "N/A".to_string();

    if let Some(request_line) = request_string.lines().next() {
        let mut parts = request_line.split_whitespace();
        if let Some(l) = parts.next(){
            method = l.to_string();
        }

        if let Some(m) = parts.next() {
            value = m.to_string();
        }
    }

    for line in request_string.lines().skip(1) {
        if line.is_empty() {
            break;
        }
        //println!("line = {}", line);
        if let Some((header_name, header_value)) = line.split_once(":") {

            match header_name.to_lowercase().as_str(){
                "host" => host = header_value.to_string(),
                "key" => key = header_value.to_string(),
                _ => {}
            }
        }
    }

    println!("[SERVER]: Recebemos um novo request =>\nMétodo: {}\nValor: {}\nHost: {}\nAccess-Key: {}", method, value, host, key);

    /*println!(
        "Request: {}", String::from_utf8_lossy(&buffer[..])
    );*/

    //let get = b"GET / HTTP/1.1\r\n";
    if key == "revproxy" {
        if method == "GET" {
            let (_, req_content) = value.split_once("/").unwrap();
            //println!("[SERVER] : Recebemos uma requisição GET!");
            let mut filepath= Path::new("404.html");
            let mut content_type = "text/html";
            if req_content.starts_with("?") {
                let (var, var_value) = req_content.split_once("=").unwrap();
                if var == "?client_file_path" {
                    filepath = Path::new(var_value);
                    content_type = "text/plain";
                }
            } else {
                
                filepath = Path::new(match req_content {
                    "" => {
                        content_type = "text/html";
                        "index.html"
                    },
                    s if s.contains("css") => {
                        content_type = "text/css";
                        req_content
                    }
                    _ => {
                        content_type = "text/plain";
                        req_content
                    }
                });
                
            }

            let contents = fs::read_to_string(filepath).unwrap();

            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
                content_type,
                contents.len(),
                contents
            );

            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        } else {
            let contents = fs::read_to_string("404.html").unwrap();

            let response = format!(
                "HTTP/1.1 404 NOT FOUND\r\nContent-Lengh: {}\r\n\r\n{}",
                contents.len(),
                contents
            );

            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }

    } else {
        let contents = fs::read_to_string("403.html").unwrap();

        let response = format!(
            "HTTP/1.1 403 FORBIDDEN\r\nContent-Lengh: {}\r\n\r\n{}",
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
        
        

    

    //Response Pattern:
    //HTTP-Version Status-Code Reason-Phrase CRLF
    //headres CRLF
    // message-body
}
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        thread::spawn(move || {
            handle_connection(stream);
        });
        

    }
}
