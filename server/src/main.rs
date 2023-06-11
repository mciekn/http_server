mod http_request;
mod http_response;
mod log;
mod request_handler;

use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 500];

    while match stream.read(&mut data) {
        Ok(size) => {
            let data = String::from_utf8_lossy(&data[0..size]);
            let request = http_request::HttpRequest::from_string(&data).unwrap();

            println!(
                "{}",
                log::received_log(
                    "INFO",
                    &request.method,
                    &request.path,
                    &stream.peer_addr().unwrap().to_string()
                )
            );

            let request_method = &request.method;

            println!(
                "{}",
                log::processing_log(
                    "INFO",
                    &request.method,
                    &request.path,
                    &stream.peer_addr().unwrap().to_string()
                )
            );

            let mut response = http_response::HttpResponse::new(
                404,
                vec![("Content-Type".to_string(), "text/html".to_string())],
                "<html><body><h1>404</h1></body></html>".to_string(),
            );

            match request_method.as_str() {
                "GET" => {
                    response = request_handler::handle_get_request(&request.path);
                }
                "POST" => {
                    response = request_handler::handle_post_request(&request.path, &request.body);
                }
                "PUT" => {
                    response = request_handler::handle_put_request(&request.path, &request.body);
                }
                "DELETE" => {
                    response = request_handler::handle_delete_request(&request.path);
                }
                _ => println!("Unknown request method"),
            }

            match stream.write(response.to_string().as_bytes()) {
                Ok(_) => {
                    println!(
                        "{}",
                        log::response_log(
                            "INFO",
                            &request.method,
                            &request.path,
                            &stream.peer_addr().unwrap().to_string(),
                            response.status_code
                        )
                    );
                }
                Err(e) => println!("Failed sending response: {}", e),
            }
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
        Err(e) => {
            println!(
                "{}",
                log::internal_server_error_log("ERROR", e.to_string(), 500)
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:2137").unwrap();
    println!("{}", log::starting_server_log("INFO", 2137));
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    drop(listener);
}
