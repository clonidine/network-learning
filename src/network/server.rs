use std::net::TcpStream;

pub fn handle_stream(stream: TcpStream) {
    let addr = stream.peer_addr();

    if let Ok(addr) = addr {
        let port = addr.port();

        println!("Connection from port: {}", port)
    }
}
