extern crate irc;

use irc::*;

fn main() {
    let mut user = IrcUser::new("nick", "username", "real name");
    user.set_password("nickserv password");

    let mut client = IrcClient::new("192.168.0.117:1337", user);
    println!("{:?}", client);
}
