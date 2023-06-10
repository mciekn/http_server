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

    pub fn to_string(&self) -> String {
        let mut request = format!("{} {}\r\n", self.method, self.path);
        for (header_name, header_value) in &self.headers {
            request.push_str(&format!("{}: {}\r\n", header_name, header_value));
        }
        request.push_str("\r\n");
        request.push_str(&self.body);
        request
    }
}
