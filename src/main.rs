mod request;
mod response;
use std::{
    io::{Read, Result, Write},
    net::{TcpListener, TcpStream},
};

fn handle_client(mut stream: TcpStream) -> Result<()> {
    let mut buffer = [0; 512];
    match stream.read(&mut buffer) {
        Ok(0) => print!("connetion closed"),
        Ok(n) => {
            stream.write(&buffer[0..n]).unwrap();
            print!("Received: {}", String::from_utf8_lossy(&buffer[0..n]));
        }
        Err(e) => eprintln!("Unexpected error: {}", e),
    }
    Ok(())
}

fn main() -> Result<()> {
    let host = "127.0.0.1:7878";
    let listener = TcpListener::bind(host)?;

    println!("HTTP server run on http://{host}");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("The stream is: {:?}", stream);
                handle_client(stream)?;
            }
            Err(e) => eprintln!("Unexpected error: {e}"),
        }
    }

    Ok(())
}
