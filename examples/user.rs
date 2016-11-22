extern crate irc;

use irc::*;
use std::io::*;

fn main() {
    let mut user = IrcUser::new("RockyTV", "yarircl", "Rust IRC Bot");
    user.set_password("1432");

    let mut client = IrcClient::new("chat.freenode.net:6667", user);
    let mut stream = client.connect();
    let mut buffer = String::new();

    let mut previous = 0;
    let mut current = 0;
    while client.conn_handler(&mut stream) {
        let ref curr_msg = client.messages[current];
        println!("{:?}", curr_msg.raw);

        if curr_msg.command == NumericReply::PRIVMSG {
            println!("{:?}", curr_msg);
            if curr_msg.params[1] == "!ping" {
                stream.send_message(&curr_msg.params[0], "Hello there!");
            }
        }

        previous = current;
        current += 1;
    }
}
