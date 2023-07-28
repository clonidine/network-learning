use std::net::TcpListener;

extern crate network;

fn main() -> std::io::Result<()> {
    let addr_format = "localhost:1337";

    let listener = TcpListener::bind(addr_format)?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => network::network::server::handle_stream(stream),

            Err(e) => panic!("{}", e),
        }
    }

    Ok(())
}
