extern crate futures;
extern crate tokio_core;
extern crate tokio_io;

use std::io::Read;
use std::collections::HashMap;

use futures::stream::Stream;
use tokio_core::reactor::Core;
use tokio_core::net::TcpListener;
use tokio_io::AsyncRead;

const LISTEN_TO: &'static str ="127.0.0.1:8001";

fn main() {
    let socket = LISTEN_TO.parse().unwrap();

    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let listener = TcpListener::bind(&socket, &handle).unwrap();
    println!("Listening on socket {}", socket);

    //  wait to parse_message until we have either reached 510 chars or \r\n
    //  create a new message after that has occurred. How do we account for >510 chars?
    //  Determine separate message by what must occur at the beginning of a message. Discard anything after
    //  the 510 char limit and before the proper beginning of a message.
    let mut message = String::new();

    let connections = listener.incoming().and_then(|(mut socket, addr)| {
        println!("New Connection: {}", addr);
        socket.read_to_string(&mut message);
        println!("SOCKET {:?}", message);
        tokio_io::io::write_all(socket, b"Hello!\n")
    });
    let server = connections.for_each( move |(mut socket, welcome)| {
        Ok(())
        });

    core.run(server).unwrap();

}

/// Separate a message into components.
fn parse_message(message: &str) -> HashMap<String, String> {
    let split = message.split(":");
    let vec = split.collect::<Vec<&str>>();
    let vec_len = vec.len();

//    is it bad form to take over the name of a variable as I've done here?
    let (prefix, rest) = separate_prefix(message);
    let (rest, usr_msg) = separate_usr_msg(rest);

    HashMap::new()
}

/// Return the given str as a tuple separated by space colon. This is to be used for separating the
/// user's portion of a message from an IRC message that has already had the prefix removed.
fn separate_usr_msg(rest: &str) -> (&str, &str) {
    let colon_loc = rest.find(" :");
    match colon_loc {
        Some(usize) => {
            let colon_loc_u = colon_loc.unwrap();
            let (rest, usr_msg) = rest.split_at(colon_loc_u + 2);
            (&rest[0..colon_loc_u], usr_msg)
        },
        None => (rest, ""),
    }
}

/// Separate the prefix from the rest of the message and return both separately.
fn separate_prefix(message: &str) -> (String, &str) {
    let prefix:String;
    let rest;

    if message.starts_with(':') {
        let message_split:Vec<&str> = message.splitn(2, " ").collect();

        // Remove the colon from the prefix.
        prefix = message_split[0].chars().skip(1).take(message_split[0].len() - 1).collect();
        // Assert error in case message_split isn't long enough?
        rest = message_split[1];
    } else {
        rest = message;
        prefix = String::new();
    }

    (prefix, rest)
}

#[cfg(test)]
mod test {
    use separate_prefix;
    use separate_usr_msg;

    #[test]
    fn test_separate_prefix() {
        let message = ":borja!borja@polaris.cs.uchicago.edu PRIVMSG #cmsc23300 :Hello everybody";
        let (prefix, rest) = separate_prefix(message);
        assert_eq!(rest, "PRIVMSG #cmsc23300 :Hello everybody");
        assert_eq!(prefix, "borja!borja@polaris.cs.uchicago.edu");

        let message2 = "QUIT :Done for the day, leaving";
        let (prefix2, rest2) = separate_prefix(message2);
        assert_eq!(rest2, "QUIT :Done for the day, leaving");
        assert_eq!(prefix2, "");

        let message3 = "WHOIS doctor";
        let (prefix3, rest3) = separate_prefix(message3);
        assert_eq!(rest3, "WHOIS doctor");
        assert_eq!(prefix3, "");
    }

    #[test]
    fn test_separate_user_msg() {
        let message = "PRIVMSG #cmsc23300 :Hello everybody";
        let (rest, usr_msg) = separate_usr_msg(message);
        println!("REST {:?}", rest);
        println!("Usr msg {:?}", usr_msg);
        assert_eq!(rest, "PRIVMSG #cmsc23300");
        assert_eq!(usr_msg, "Hello everybody");

        let message2 = "WHOIS doctor";
        let (rest2, usr_msg2) = separate_usr_msg(message2);
        println!("REST {:?}", rest2);
        println!("Usr msg {:?}", usr_msg2);
        assert_eq!(rest2, "WHOIS doctor");
        assert_eq!(usr_msg2, "");

    }
}
