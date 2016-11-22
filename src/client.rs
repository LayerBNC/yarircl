use user::IrcUser;
use message::IrcMessage;
use std::net::{ToSocketAddrs, TcpStream};
use std::io::*;
use std::str::FromStr;
use bufstream::BufStream;

#[derive(Debug)]
pub struct IrcClient<A: ToSocketAddrs> {
    pub server: A,
    pub user: IrcUser,
    pub connected: bool,
    pub messages: Vec<IrcMessage>
}

impl <A: ToSocketAddrs> IrcClient<A> {
    pub fn new(server: A, user: IrcUser) -> IrcClient<A> {
        let mut client = IrcClient {
            server: server,
            user: user,
            connected: false,
            messages: Vec::new()
        };

        return client;
    }

    pub fn connect(&mut self) -> BufStream<TcpStream> {
        let stream = TcpStream::connect(&self.server).unwrap();
        let mut bufstream = BufStream::new(stream);

        bufstream.write(&format!("NICK {}\r\n", self.user.nick).as_bytes()).unwrap();
        bufstream.write(&format!("USER {} 0 * :{}\r\n", self.user.user, self.user.real_name).as_bytes()).unwrap();
        bufstream.flush();

        self.connected = true;
        return bufstream;
    }

    pub fn conn_handler(&mut self, stream: &mut BufStream<TcpStream>) -> bool {
        let mut buffer = String::new();
        if stream.read_line(&mut buffer).unwrap() > 0 {
            let message = match IrcMessage::from_str(&buffer) {
                Ok(x) => x,
                Err(e) => return false
            };
            self.messages.push(message);
            return true;
        } 
        else {
            self.connected = false;
            return false;
        }
    }
}
