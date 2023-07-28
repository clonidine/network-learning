use std::net::TcpStream;

pub fn connect(address: &str) {
    let client = TcpStream::connect(address);

    match client {
        Ok(_) => println!("Connected."),
        Err(err) => panic!("Cannot connect: {}", err),
    }
}
