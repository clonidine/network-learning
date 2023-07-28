use std::net::TcpListener;

extern crate network;

fn main() -> std::io::Result<()> {
    let addr = "localhost:1337";

    let listener = TcpListener::bind(addr)?;

    loop {
        let _ = network::network::client::connect(addr);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => network::network::server::handle_stream(stream),

                Err(e) => panic!("{}", e),
            }
        }
    }
}
