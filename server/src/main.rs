use std::thread;
use std::process;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::thread::Thread;
use thread_id;
use chrono::prelude::*;

fn handle_client(mut stream: TcpStream){
    let mut data = [0 as u8; 500];
    println!("My pid is {}", process::id());
    println!("My thread id is {}",thread_id::get());
    while match stream.read(&mut data){
        Ok(size) => {
            let data = String::from_utf8_lossy(&data[0..size]);
            println!("Received data: {}", data);

            let request = HttpRequest::from_string(&data).unwrap();

            println!("Request method: {}", request.method);
            println!("Request path: {}", request.path);
            println!("Request headers: {:?}", request.headers);
            println!("Request body: {}", request.body);

            let logLine = create_log_string("INFO", &request.method, &request.path, &stream.peer_addr().unwrap().to_string());
            println!("{}", logLine);


            let requestMethod = request.method;

            match requestMethod.as_str() {
                "GET" => println!("GET"),
                "POST" => println!("POST"),
                "PUT" => println!("PUT"),
                "DELETE" => println!("DELETE"),
                _ => println!("Unknown request method")
            }

            let response = HttpResponse::new(
                200,
                vec![("Content-Type".to_string(), "text/html".to_string())],
                "<html><body><h1>Hello World!</h1></body></html>".to_string(),
            );




            //let response = "HTTP/1.1 200 OK\r\n\r\n<html><body><h1>Hello World!</h1></body></html>";

            match stream.write(response.to_string().as_bytes()) {
                Ok(_) => println!("Response sent"),
                Err(e) => println!("Failed sending response: {}", e)
            }
            //stream.shutdown(Shutdown::Both).unwrap();
            false
        },
        Err(_) => {
            println!("An error occured, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}



fn main() {
    let listener = TcpListener::bind("127.0.0.1:2137").unwrap();
    println!("Server listening on port 2137");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    drop(listener);
}

fn create_log_string(level: &str, method: &str, path: &str, ip: &str) -> String {
    let timestamp = Local::now().format("[%Y-%m-%d %H:%M:%S]");

    format!(
        "{} {}: Request received - Method: {}, Path: {}, IP: {}",
        timestamp, level, method, path, ip
    )
}


struct HttpResponse {
    status_code: u16,
    headers: Vec<(String, String)>,
    body: String,
}

impl HttpResponse {
    fn new(status_code: u16, headers: Vec<(String, String)>, body: String) -> Self {
        HttpResponse {
            status_code,
            headers,
            body,
        }
    }

    fn to_string(&self) -> String {
        let mut response = format!("HTTP/1.1 {}\r\n", self.status_code);
        for (header_name, header_value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", header_name, header_value));
        }
        response.push_str("\r\n");
        response.push_str(&self.body);
        response
    }

}

/// HTTP request representation
/// TODO: MOVE TO ANOTHER FILE

struct HttpRequest {
    method: String,
    path: String,
    headers: Vec<(String, String)>,
    body: String,
}

impl HttpRequest {
    fn new(method: String, path: String, headers: Vec<(String, String)>, body: String) -> Self {
        HttpRequest {
            method,
            path,
            headers,
            body,
        }
    }

    fn from_string(request_string: &str) -> Option<Self> {
        let mut lines = request_string.lines();

        if let Some(request_line) = lines.next() {
            let mut parts = request_line.split_whitespace();
            let method = parts.next()?.to_string();
            let path = parts.next()?.to_string();

            let mut headers = Vec::new();
            while let Some(header_line) = lines.next() {
                if header_line.is_empty() {
                    break;
                }
                let mut header_parts = header_line.splitn(2, ": ");
                let header_name = header_parts.next()?.to_string();
                let header_value = header_parts.next()?.to_string();
                headers.push((header_name, header_value));
            }

            let body = lines.collect::<Vec<&str>>().join("\n");

            Some(HttpRequest::new(method, path, headers, body))
        } else {
            None
        }
    }
}
