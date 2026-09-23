mod request;
mod response;
mod thread_pool;
use std::{
    io::Result,
    net::{TcpListener, TcpStream},
};

use crate::{request::Request, response::Response, thread_pool::ThreadPool};

fn main() -> Result<()> {
    let host = "0.0.0.0:7878";
    let listener = TcpListener::bind(host)?;
    let pool = ThreadPool::new(4);

    println!("HTTP server run on http://{host}");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                pool.execute(|| {
                    if let Err(e) = handle_connection(stream) {
                        eprintln!("connection error: {e}");
                    }
                });
            }
            Err(e) => eprintln!("Unexpected error: {e}"),
        }
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let request = Request::parse(&mut stream)?;
    println!("{} {}", request.method, request.path);

    let mut response = route(&request);
    response.write_to(&mut stream)
}

fn route(request: &Request) -> Response {
    match (request.method.as_str(), request.path.as_str()) {
        ("GET", "/") => {
            let mut res = Response::ok();
            res.set_html("<h1>Hello from the server!!!</h1>");
            res
        }
        ("GET", "/health") => {
            let mut res = Response::ok();
            res.set_json(r#"{"status": "ok"}"#);
            res
        }
        ("POST", "/echo") => {
            let mut res = Response::ok();
            let body = String::from_utf8_lossy(&request.body).to_string();
            res.set_text(&body);
            res
        }
        _ => Response::not_found(),
    }
}
