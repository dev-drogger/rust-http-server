// use http::{Request, Response};
use std::net::{TcpListener, TcpStream};

fn main() -> std::io::Result<()> {
    let host = "127.0.0.1:7878";
    let listener = TcpListener::bind(host)?;

    println!("HTTP server run on http://{host}");

    for stream in listener.incoming() {
        println!("The stream is: {:?}", stream)
    }

    Ok(())
}
