pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

impl HttpRequest {
    pub fn new(method: String, path: String, headers: Vec<(String, String)>, body: String) -> Self {
        HttpRequest {
            method,
            path,
            headers,
            body,
        }
    }

    pub fn from_string(request_string: &str) -> Option<Self> {
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

