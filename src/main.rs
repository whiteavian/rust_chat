// Simpler example:
// https://medium.com/adventures-in-rust/moving-to-tcpstream-bye-tokio-5a1488f337f6
use std::collections::HashMap;
use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8001").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = String::new();
                stream.read_to_string(&mut buffer);
                parse_message(&buffer);
                stream.write(buffer.as_bytes()).expect("Response failed.");
            }
            Err(e) => {
                println!("Unable to connect: {}.", e);
            }
        }
    }
}

fn parse_message(message: &str) -> HashMap<String, String> {
    println!("MESSAGE {:?}", message);
    let mut split = message.split(":");
    //    TODO I do not completely understand this syntax.
    let mut vec = split.collect::<Vec<&str>>();
    let vec_len = vec.len();

    let rest;
    let usr_msg = "";
    let prefix;

    // TODO make prefix an option?
    if vec[0] == "" {
        let prefix_rest_split = vec[1].split(" ");
        let mut prefix_rest_vec = prefix_rest_split.collect::<Vec<&str>>();

        prefix = prefix_rest_vec[0];
        rest = prefix_rest_vec.split_at(1).1;
    } else {
        prefix = "";
        rest = vec;
    }
    println!("rest {:?}", rest);
    println!("prefix {:?}", prefix);
    println!("usr_msg {:?}", usr_msg);
    //    if vec.len() == 2 {
    //    } else if vec.len() == 3 {
    //
    //    }
    HashMap::new()
}


