mod HttpRequest;
mod HttpResponse;
mod Log;
mod RequestHandler;

use std::fmt::Error;
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
            let request = HttpRequest::HttpRequest::from_string(&data).unwrap();

            /*
            println!("My pid is {}", process::id());
            println!("My thread id is {}",thread_id::get());
            println!("Request method: {}", request.method);
            println!("Request path: {}", request.path);
            println!("Request headers: {:?}", request.headers);
            println!("Request body: {}", request.body);
            */

            println!("{}", Log::received_log("INFO", &request.method, &request.path, &stream.peer_addr().unwrap().to_string()));


            let requestMethod = &request.method;

            println!("{}", Log::processing_log("INFO", &request.method, &request.path, &stream.peer_addr().unwrap().to_string()));

            let mut response= HttpResponse::HttpResponse::new(
                404,
                vec![("Content-Type".to_string(), "text/html".to_string())],
                "<html><body><h1>404</h1></body></html>".to_string(),
            );

            match requestMethod.as_str() {
                "GET" => {
                    response = RequestHandler::handle_get_request(&request.path);
                },
                "POST" => {
                    response = RequestHandler::handle_post_request(&request.path, &request.body);
                },
                "PUT" => {
                    response = RequestHandler::handle_put_request(&request.path, &request.body);
                },
                "DELETE" => {
                    response = RequestHandler::handle_delete_request(&request.path);
                }
                _ => println!("Unknown request method")
            }

            match stream.write(response.to_string().as_bytes()) {
                Ok(_) => {
                    let logLine = Log::response_log("INFO", &request.method, &request.path, &stream.peer_addr().unwrap().to_string(), response.status_code);
                    println!("{}", logLine)
                },
                Err(e) => println!("Failed sending response: {}", e)
            }
            stream.shutdown(Shutdown::Both).unwrap();
            false
        },
        Err(e) => {
            println!("{}", Log::internal_server_error_log("ERROR", e.to_string(), 500));
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}



fn main() {
    let listener = TcpListener::bind("127.0.0.1:2137").unwrap();
    println!("{}", Log::starting_server_log("INFO", 2137));
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
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









