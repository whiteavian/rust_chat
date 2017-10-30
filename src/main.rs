extern crate futures;
extern crate tokio_core;
extern crate tokio_io;

use futures::{Future, Stream};
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;
use tokio_io::io::copy;
use tokio_io::AsyncRead;


// This is more or less identical to the example provided by:
// https://tokio-rs.github.io/tokio-core/tokio_core/index.html
fn main() {

    let mut event_loop = Core::new().unwrap();
    let handle = event_loop.handle();
//    let address = "127.0.0.1:8001".parse::<SocketAddr>().unwrap();
    let address = "127.0.0.1:8001".parse().unwrap();
    let server_socket = TcpListener::bind(&address, &handle).unwrap();

    let server = server_socket.incoming().for_each(|(sock, _)| {
        let (reader, writer) = sock.split();

        let bytes_copied = copy(reader, writer);

        let handle_conn = bytes_copied.map(|amt| {
            println!("wrote {:?} bytes", amt)
        }).map_err(|err| {
            println!("IO error {:?}", err)
        });

        handle.spawn(handle_conn);

        Ok(())
    });

    event_loop.run(server).unwrap();
}
