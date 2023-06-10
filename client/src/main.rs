mod http_request;

use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;
use crate::http_request::HttpRequest;

fn main(){
    match TcpStream::connect("127.0.0.1:2137"){
        Ok(mut stream) => {
            println!("Successfully connected to server in port 2137");

            let msg = http_request::HttpRequest::new(
                "GET".to_string(),
                "/".to_string(),
                vec![("Host".to_string(), "localhost".to_string())],
                "".to_string()
            ).to_string().into_bytes();

            stream.write(&*msg).unwrap();

            let mut data = [0 as u8; 170];
            match stream.read_exact(&mut data){
                Ok(_) => {
                    if true {
                        let text = from_utf8(&data).unwrap();
                        println!("Reply: {}", text);
                    }
                    else{
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {}", text);
                    }
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }

        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.")
}