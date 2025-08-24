use std::fmt::format;
use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::thread;
use std::path::Path;
use rand::Rng;
use sha2::{Sha256, Digest};
use std::sync::Arc;


fn handle_connection(mut stream: TcpStream, secret: Arc<String>) {
    let mut buffer = [0; 4096];
    stream.read(&mut buffer).unwrap();
    
    let request_string = String::from_utf8_lossy(&buffer[..]);

    let mut method = "N/A".to_string();
    let mut value = "N/A".to_string();
    let mut host = "N/A".to_string();
    let mut key = "N/A".to_string();

    if let Some(request_line) = request_string.lines().next() {
        let (_, proxy_secret) = request_line.split_once(": ").unwrap();
        key = proxy_secret.to_string();
    }

    if let Some(request_line) = request_string.lines().skip(1).next() {
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
                _ => {}
            }
        }
    }

    println!("[SERVER]: Recebemos um novo request =>\nMétodo:{}\nValor:{}\nHost:{}\nAccess-Key:{}", method, value, host, key);

    /*println!(
        "Request: {}", String::from_utf8_lossy(&buffer[..])
    );*/

    //let get = b"GET / HTTP/1.1\r\n";
    dbg!(&secret);
    dbg!(&key);
    if key == secret.as_str() {
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

            let response;
            if filepath.exists() {
                let contents = fs::read_to_string(filepath).unwrap();

                response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
                    content_type,
                    contents.len(),
                    contents
                );
            } else {
                let contents = fs::read_to_string("404.html").unwrap();

                response = format!(
                    "HTTP/1.1 404 NOT FOUND\r\nContent-Lengh: {}\r\n\r\n{}",
                    contents.len(),
                    contents
                );
            }

            

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
    const CHARSET_SENHA_FORTE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                        abcdefghijklmnopqrstuvwxyz\
                                        0123456789\
                                        !@#$%^&*()_+-=[]{}|;:,.<>?";

    let secret_string = gerar_secret_string(16, CHARSET_SENHA_FORTE);

    let mut hasher = Sha256::new();
    hasher.update(secret_string.as_bytes());
    let hash_byted = hasher.finalize();
    let secret_key = hex::encode(hash_byted);

    println!("[SERVER]: Generated secret_key: {}", secret_key);

    if let Err(e) = register_with_proxy(&secret_key.as_str()) {
        eprintln!("[SERVER]: Critical Error: {}", e);
    }

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let secret_key_arc = Arc::new(secret_key);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let secret_clone = Arc::clone(&secret_key_arc);
        thread::spawn(move || {
            handle_connection(stream, secret_clone);
        });
        

    }
}

fn gerar_secret_string(tamanho: usize, charset: &[u8]) -> String {
    let mut rng = rand::rng();
    (0..tamanho).map(|_| {
        let indice = rng.random_range(0..charset.len());
        charset[indice] as char
    }).collect()
}

fn register_with_proxy(secret: &str) -> Result<(), String> {
    println!("[SERVER]: Trying to register secret_key!");
    match TcpStream::connect("0.0.0.0:3000") {
        Ok(mut stream) => {
            let request = format!(
                "POST /register-secret HTTP/1.1\r\n\
                Host: 127.0.0.1:7878\r\n\
                Content-Type: text/plain\r\n\
                Content-Length: {}\r\n\
                \r\n\
                {}",
                secret.len(),
                secret
            );

            stream.write(request.as_bytes()).unwrap();

            let mut response_buffer = [0; 512];
            stream.read(&mut response_buffer).map_err(|e| e.to_string())?;
            let response_str = String::from_utf8_lossy(&response_buffer);

            if response_str.starts_with("HTTP/1.1 200 OK") {
                println!("[SERVER]: Secret-key has been setted up!");
                Ok(())
            } else {
                Err(format!("Erro no registro. Resposta do Proxy:\n{}", response_str))
            }
        }
        Err(e) => Err(format!("Falha ao se conectar com o Proxy: {}", e))
    }

    
}
