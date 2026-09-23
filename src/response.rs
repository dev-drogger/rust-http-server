use std::collections::HashMap;
use std::io::{Result, Write};
use std::net::TcpStream;

pub struct Response {
    pub status: u16,
    pub reason: &'static str,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Response {
    pub fn new(status: u16, reason: &'static str) -> Response {
        Response {
            status,
            reason,
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }

    pub fn ok() -> Response {
        Response::new(200, "Ok")
    }

    pub fn not_found() -> Response {
        let mut res = Response::new(404, "Not Found");
        res.set_text("404 Not Found");
        res
    }

    pub fn set_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    pub fn set_text(&mut self, text: &str) {
        self.body = text.as_bytes().to_vec();
        self.set_header("Content-Type", "text/plain; charset=utf-8");
    }

    pub fn set_html(&mut self, html: &str) {
        self.body = html.as_bytes().to_vec();
        self.set_header("Content-Type", "text/html; charset=utf-8");
    }

    pub fn set_json(&mut self, json: &str) {
        self.body = json.as_bytes().to_vec();
        self.set_header("Content-Type", "application/json");
    }

    pub fn write_to(&mut self, stream: &mut TcpStream) -> Result<()> {
        self.headers
            .entry("Content-Length".to_string())
            .or_insert_with(|| self.body.len().to_string());
        self.headers
            .entry("Connection".to_string())
            .or_insert_with(|| "close".to_string());

        let mut response = format!("HTTP/1.1 {} {}\r\n", self.status, self.reason);

        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }
        response.push_str("\r\n");
        stream.write_all(response.as_bytes());
        stream.write_all(&self.body)?;
        stream.flush()
    }
}
