use chrono::prelude::*;

pub fn starting_server_log(level: &str, port: u16) -> String {
    let timestamp = Local::now().format("[%Y-%m-%d %H:%M:%S]");

    format!(
        "{} {}: Starting server on port {}",
        timestamp, level, port
    )
}

pub fn received_log(level: &str, method: &str, path: &str, ip: &str) -> String {
    let timestamp = Local::now().format("[%Y-%m-%d %H:%M:%S]");

    format!(
        "{} {}: Request received - Method: {}, Path: {}, IP: {}",
        timestamp, level, method, path, ip
    )
}

pub fn processing_log(level: &str, method: &str, path: &str, ip: &str) -> String {
    let timestamp = Local::now().format("[%Y-%m-%d %H:%M:%S]");

    format!(
        "{} {}: Processing request - Method: {}, Path: {}, IP: {}",
        timestamp, level, method, path, ip
    )
}

pub fn response_log(level: &str, method: &str, path: &str, ip: &str, status_code: u16) -> String {
    let timestamp = Local::now().format("[%Y-%m-%d %H:%M:%S]");

    format!(
        "{} {}: Request processed successfully - Method: {}, Path: {}, IP: {}, Status code: {}",
        timestamp, level, method, path, ip, status_code
    )
}

pub fn internal_server_error_log(level: &str, error: String, status_code: u16) -> String {
    let timestamp = Local::now().format("[%Y-%m-%d %H:%M:%S]");

    format!(
        "{} {}: {} Internal server error - Error: {}",
        timestamp, level, status_code, error
    )
}


