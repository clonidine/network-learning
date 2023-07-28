use std::net::{TcpListener, TcpStream};

const CONNECTION_ADDR: &str = "localhost";
const CONNECTION_PORT: u16 = 1337;

fn handle_stream(stream: TcpStream) {
    let addr = stream.peer_addr();

    if let Ok(addr) = addr {
        let port = addr.port();

        println!("Connection from port: {}", port)
    }
}

fn main() -> std::io::Result<()> {
    let addr_format = format!("{}:{}", CONNECTION_ADDR, CONNECTION_PORT);

    let listener = TcpListener::bind(addr_format)?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_stream(stream),

            Err(e) => panic!("{}", e),
        }
    }

    Ok(())
}
