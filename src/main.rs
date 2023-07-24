use std::{net::TcpListener, io::ErrorKind};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000");

    let listener = match listener {
        Ok(tcplistener) => tcplistener,
        Err(error) => match error.kind() {
            ErrorKind::PermissionDenied => panic!("You need sudo rights {:?}", error),
            _ => panic!("You got a other error {:?}", error),
        },
    };

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established");
    }
}
