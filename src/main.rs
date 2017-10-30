//extern crate futures;
//extern crate tokio_core;
//extern crate tokio_io;
//
//use futures::{Future, Stream};
//use tokio_core::net::TcpListener;
//use tokio_core::reactor::Core;
//use tokio_io::io::{read_to_end, write_all};
//use tokio_io::AsyncRead;
//
//
//// This is more or less identical to the example provided by:
//// https://tokio-rs.github.io/tokio-core/tokio_core/index.html
//fn main() {
//
//    let mut core = Core::new().unwrap();
//    let handle = core.handle();
//    let address = "127.0.0.1:8001".parse().unwrap();
//    let server_socket = TcpListener::bind(&address, &handle).unwrap();
//
//    let server = server_socket.incoming().for_each(|(sock, _)| {
////        let (reader, writer) = sock.split();
////        let result = sock.read_to_string();
//
//        Ok(())
//    });
//
//    core.run(server).unwrap();
//}


// Simpler example:
// https://medium.com/adventures-in-rust/moving-to-tcpstream-bye-tokio-5a1488f337f6
use std::net::{TcpStream, TcpListener};
use std::io::Write;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8001").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let response = b"Hello World!\n";
                stream.write(response).expect("Response failed.");
            }
            Err(e) => {
                println!("Unable to connect: {}.", e);
            }
        }
    }
}
