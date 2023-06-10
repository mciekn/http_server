pub struct HttpResponse {
    pub status_code: u16,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

impl HttpResponse {
    pub fn new(status_code: u16, headers: Vec<(String, String)>, body: String) -> Self {
        HttpResponse {
            status_code,
            headers,
            body,
        }
    }

    pub fn to_string(&self) -> String {
        let mut response = format!("HTTP/1.1 {}\r\n", self.status_code);
        for (header_name, header_value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", header_name, header_value));
        }
        response.push_str("\r\n");
        response.push_str(&self.body);
        response
    }

}