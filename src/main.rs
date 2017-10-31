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
                let mut message = String::new();
//  wait to parse_message until we have either reached 510 chars or \r\n
//  create a new message after that has occurred. How do we account for >510 chars?
//  Determine separate message by what must occur at the beginning of a message. Discard anything after
//  the 510 char limit and before the proper beginning of a message.
                stream.read_to_string(&mut message);
                parse_message(&message);
                stream.write(message.as_bytes()).expect("Response failed.");
            }
            Err(e) => {
                println!("Unable to connect: {}.", e);
            }
        }
    }
}

/// Separate a message into components.
fn parse_message(message: &str) -> HashMap<String, String> {
    println!("MESSAGE {:?}", message);
    let mut split = message.split(":");
    //    TODO I do not completely understand this syntax.
    let mut vec = split.collect::<Vec<&str>>();
    let vec_len = vec.len();

    let rest: Vec<&str>;
    let usr_msg = "";
    let prefix;

    // TODO make prefix an option?
    if vec[0] == "" {
        let foo: &str;
//        let prefix_rest_split = vec[1].split(" ");
//        let mut prefix_rest_vec = prefix_rest_split.collect::<Vec<&str>>();
//
//        prefix = prefix_rest_vec[0];
//        rest = prefix_rest_vec.split_at(1).1;
//        println!("rest {:?}", rest);
    } else {
        prefix = "";
//        rest = vec;
    }
//    println!("prefix {:?}", prefix);
    println!("usr_msg {:?}", usr_msg);
    //    if vec.len() == 2 {
    //    } else if vec.len() == 3 {
    //
    //    }
    HashMap::new()
}

/// Extract the prefix from the message and return the prefix or an empty string.
fn extract_prefix(message: &str) -> (String, &str) {
    let prefix:String;
    let mut rest = "";

    if message.starts_with(':') {
        let mut message_split:Vec<&str> = message.splitn(2, " ").collect();

//        Remove the colon from the prefix.
        prefix = message_split[0].chars().skip(1).take(message_split[0].len() - 1).collect();

        if message_split.len() == 2 {
            rest = message_split[1];
        } else {
//            assert error here instead, because there should always be a space and therefore a message?
        }
    } else {
        rest = message;
        prefix = String::new();
    }

    (prefix, rest)
}

#[cfg(test)]
mod test {
    use extract_prefix;

    #[test]
    fn test_extract_prefix() {
        let mut message = ":borja!borja@polaris.cs.uchicago.edu PRIVMSG #cmsc23300 :Hello everybody";
        let (prefix, rest) = extract_prefix(message);
        assert_eq!(rest, "PRIVMSG #cmsc23300 :Hello everybody");
        assert_eq!(prefix, "borja!borja@polaris.cs.uchicago.edu");

        let mut message2 = "QUIT :Done for the day, leaving";
        let (prefix2, rest2) = extract_prefix(message2);
        assert_eq!(rest2, "QUIT :Done for the day, leaving");
        assert_eq!(prefix2, "");

        let mut message3 = "WHOIS doctor";
        let (prefix3, rest3) = extract_prefix(message3);
        assert_eq!(rest3, "WHOIS doctor");
        assert_eq!(prefix3, "");
    }
}
