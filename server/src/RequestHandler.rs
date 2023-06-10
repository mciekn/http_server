use crate::HttpResponse;

pub fn handle_get_request(path: &str) -> HttpResponse::HttpResponse {
    let mut response =
        HttpResponse::HttpResponse::new(
            404,
            vec![("Content-Type".to_string(), "text/html".to_string())],
            "<html><body><h1>404</h1></body></html>".to_string(),
        );

    match path {
        "/" => {
            response =
                HttpResponse::HttpResponse::new(
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
                HttpResponse::HttpResponse::new(
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

pub fn handle_post_request(path: &str, body: &str) -> HttpResponse::HttpResponse {
    let mut response =
        HttpResponse::HttpResponse::new(
            404,
            vec![("Content-Type".to_string(), "text/html".to_string())],
            "<html><body><h1>404</h1></body></html>".to_string(),
        );
    match path {
        "/contact" => {
            response = HttpResponse::HttpResponse::new(
                200,
                vec![("Content-Type".to_string(), "text/html".to_string())],
                "<html><body><h1>Thank you for contacting us!</h1></body></html>".to_string(),
            );
        }
        _ => {}
    }
    response
}

pub fn handle_put_request(path: &str, body: &str) -> HttpResponse::HttpResponse {
    let mut response =
        HttpResponse::HttpResponse::new(
            404,
            vec![("Content-Type".to_string(), "text/html".to_string())],
            "<html><body><h1>404</h1></body></html>".to_string(),
        );
    match path {
        "/contact" => {
            response = HttpResponse::HttpResponse::new(
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

pub fn handle_delete_request(path: &str) -> HttpResponse::HttpResponse {
    let mut response =
        HttpResponse::HttpResponse::new(
            404,
            vec![("Content-Type".to_string(), "text/html".to_string())],
            "<html><body><h1>404</h1></body></html>".to_string(),
        );
    match path {
        "/contact" => {
            response = HttpResponse::HttpResponse::new(
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