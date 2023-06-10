use std::thread;
use std::process;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::thread::Thread;
use thread_id;
use chrono::prelude::*;

fn handle_client(mut stream: TcpStream){
    let mut data = [0 as u8; 500];

    while match stream.read(&mut data){
        Ok(size) => {
            let data = String::from_utf8_lossy(&data[0..size]);

            let request = HttpRequest::from_string(&data).unwrap();

            /*
            println!("My pid is {}", process::id());
            println!("My thread id is {}",thread_id::get());
            println!("Request method: {}", request.method);
            println!("Request path: {}", request.path);
            println!("Request headers: {:?}", request.headers);
            println!("Request body: {}", request.body);
            */

            let logLine = received_log_string("INFO", &request.method, &request.path, &stream.peer_addr().unwrap().to_string());
            println!("{}", logLine);


            let requestMethod = &request.method;

            let logLine = processing_log_string("INFO", &request.method, &request.path, &stream.peer_addr().unwrap().to_string());
            println!("{}", logLine);

            let mut response= HttpResponse::new(
                404,
                vec![("Content-Type".to_string(), "text/html".to_string())],
                "<html><body><h1>404</h1></body></html>".to_string(),
            );

            match requestMethod.as_str() {
                "GET" => {
                    response = handle_get_request(&request.path);
                },
                "POST" => {
                    response = handle_post_request(&request.path, &request.body);
                },
                "PUT" => {
                    response = handle_put_request(&request.path, &request.body);
                },
                "DELETE" => {
                    response = handle_delete_request(&request.path);
                }
                _ => println!("Unknown request method")
            }

            match stream.write(response.to_string().as_bytes()) {
                Ok(_) => {
                    let logLine = response_log_string("INFO", &request.method, &request.path, &stream.peer_addr().unwrap().to_string(), response.status_code);
                    println!("{}", logLine)
                },
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
                //println!("New connection: {}", stream.peer_addr().unwrap());
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

fn handle_get_request(path: &str) -> HttpResponse {
    let mut response =
        HttpResponse::new(
            404,
            vec![("Content-Type".to_string(), "text/html".to_string())],
            "<html><body><h1>404</h1></body></html>".to_string(),
        );

    match path {
        "/" => {
            response =
                HttpResponse::new(
                200,
                vec![("Content-Type".to_string(), "text/html".to_string())],
                "<html>\
                <body><h1>Welcome on our sample page!</h1></body>\
                <a href=\"/about\">About</a></body>\
                <br><a href=\"/contact\">Contact</a></body></html>".to_string(),
            );
        }
        "/about" => {
            response =
                HttpResponse::new(
                200,
                vec![("Content-Type".to_string(), "text/html".to_string())],
                "<html><body><h1>About me</h1></body></html>".to_string(),
            );
        }
        _ => {
        }
    }
    response
}

fn handle_post_request(path: &str, body: &str) -> HttpResponse {
    let mut response =
        HttpResponse::new(
            404,
            vec![("Content-Type".to_string(), "text/html".to_string())],
            "<html><body><h1>404</h1></body></html>".to_string(),
        );
    match path {
        "/contact" => {
            response = HttpResponse::new(
                200,
                vec![("Content-Type".to_string(), "text/html".to_string())],
                "<html><body><h1>Thank you for contacting us!</h1></body></html>".to_string(),
            );
        }
        _ => {}
    }
    response
}

fn handle_put_request(path: &str, body: &str) -> HttpResponse {
    let mut response =
        HttpResponse::new(
            404,
            vec![("Content-Type".to_string(), "text/html".to_string())],
            "<html><body><h1>404</h1></body></html>".to_string(),
        );
    match path {
        "/contact" => {
            response = HttpResponse::new(
                200,
                vec![("Content-Type".to_string(), "text/html".to_string())],
                "<html><body><h1>Thank you for updating your contact information!</h1></body></html>".to_string(),
            );
        }
        _ => {
        }
    }
    response
}

fn handle_delete_request(path: &str) -> HttpResponse {
    let mut response =
        HttpResponse::new(
            404,
            vec![("Content-Type".to_string(), "text/html".to_string())],
            "<html><body><h1>404</h1></body></html>".to_string(),
        );
    match path {
        "/contact" => {
            response = HttpResponse::new(
                200,
                vec![("Content-Type".to_string(), "text/html".to_string())],
                "<html><body><h1>We are sorry to see you go!</h1></body></html>".to_string(),
            );
        }
        _ => {
        }
    }
    response
}

fn received_log_string(level: &str, method: &str, path: &str, ip: &str) -> String {
    let timestamp = Local::now().format("[%Y-%m-%d %H:%M:%S]");

    format!(
        "{} {}: Request received - Method: {}, Path: {}, IP: {}",
        timestamp, level, method, path, ip
    )
}

fn processing_log_string(level: &str, method: &str, path: &str, ip: &str) -> String {
    let timestamp = Local::now().format("[%Y-%m-%d %H:%M:%S]");

    format!(
        "{} {}: Processing request - Method: {}, Path: {}, IP: {}",
        timestamp, level, method, path, ip
    )
}

fn response_log_string(level: &str, method: &str, path: &str, ip: &str, status_code: u16) -> String {
    let timestamp = Local::now().format("[%Y-%m-%d %H:%M:%S]");

    format!(
        "{} {}: Request processed successfully - Method: {}, Path: {}, IP: {}, Status code: {}",
        timestamp, level, method, path, ip, status_code
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
