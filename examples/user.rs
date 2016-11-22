extern crate irc;

use irc::*;
use std::io::*;

fn main() {
    let mut user = IrcUser::new("rustybot1337", "yarircl", "Rust IRC Bot");
    user.set_password("");

    let mut client = IrcClient::new("chat.freenode.net:6667", user);
    let mut stream = client.connect();
    let mut buffer = String::new();

    /*while stream.read_line(&mut buffer).unwrap() > 0 {
        let msg = match IrcMessage::from_str(&buffer) {
            Ok(x) => println!(">> {}", x.raw),
            Err(e) => println!("error: {}", e)
        };
        buffer.clear();
    }*/

    /*loop {
        let messages = match client.get_messages(&mut stream) {
            Some(msgs) => {
                for msg in msgs {
                    println!("{}", msg.raw);
                }
            },
            None => break
        };
    }*/

    let mut previous = 0;
    let mut current = 0;
    while client.conn_handler(&mut stream) {
        println!("{:?}", client.messages[current].raw);
        
        previous = current;
        current += 1;
    }
}
