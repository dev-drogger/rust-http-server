use ::std::io::{BufRead, BufReader, Read, Result};
use std::collections::HashMap;
use std::net::TcpStream;

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Request {
    pub fn parse(stream: &mut TcpStream) -> Result<Request> {
        let mut reader = BufReader::new(stream.try_clone()?);

        let mut request_line = String::new();
        reader.read_line(&mut request_line)?;
        let mut parts = request_line.trim().split_whitespace();
        let method = parts.next().unwrap_or("").to_string();
        let path = parts.next().unwrap_or("/").to_string();
        let version = parts.next().unwrap_or("HTTP/1.1").to_string();

        let mut headers = HashMap::new();
        loop {
            let mut line = String::new();
            let bytes_read = reader.read_line(&mut line);

            if bytes_read == 0 || line.trim().is_empty() {
                break;
            }
            if let Some((key, value)) = line.split_once(":") {
                headers.insert(key.trim().to_ascii_lowercase(), value.trim().to_string());
            }
        }

        let content_length: usize = headers
            .get("content-length")
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);
        let mut body = vec![0u8; content_length];
        if content_length > 0 {
            reader.read_exact(&mut body);
        }

        Ok(Request {
            method,
            path,
            version,
            headers,
            body,
        })
    }
}
