use user::IrcUser;
use message::{IrcMessage, NumericReply};
use std::net::{ToSocketAddrs, TcpStream};
use std::io::*;
use std::str::FromStr;
use bufstream::BufStream;

#[derive(Debug)]
pub struct IrcClient<A: ToSocketAddrs> {
    pub server: A,
    pub user: IrcUser,
    pub connected: bool,
    pub messages: Vec<IrcMessage>,
    pub server_motd: String
}

impl <A: ToSocketAddrs> IrcClient<A> {
    pub fn new(server: A, user: IrcUser) -> IrcClient<A> {
        let mut client = IrcClient {
            server: server,
            user: user,
            connected: false,
            messages: Vec::new(),
            server_motd: String::new()
        };

        return client;
    }

    pub fn connect(&mut self) -> BufStream<TcpStream> {
        self.server_motd = String::new();
        self.messages.clear();

        let stream = TcpStream::connect(&self.server).unwrap();
        let mut bufstream = BufStream::new(stream);

        bufstream.send_raw_message(&format!("NICK {}\r\n", self.user.nick));
        bufstream.send_raw_message(&format!("USER {} 0 * :{}\r\n", self.user.user, self.user.real_name));
        if self.user.nickserv_password.len() > 0 {
            bufstream.identify("nickserv", &self.user.nickserv_password);
        }

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

            if message.command == NumericReply::RPL_MOTD {
                //println!("{:?}", message);
                self.server_motd.push_str(&message.params[1]);
                self.server_motd.push_str("\r\n");
            }

            if message.command == NumericReply::PING {
                let reply = &format!("PONG :{reply}", reply=message.params[0]);
                stream.send_raw_message(reply);
            }

            self.messages.push(message);
            return true;
        } 
        else {
            self.connected = false;
            return false;
        }
    }
}

pub trait IrcWrite {
    fn send_raw_message(&mut self, msg: &str) -> Result<usize>;
    fn send_message(&mut self, destination: &str, msg: &str) -> Result<usize>;
    fn identify(&mut self, ns_name: &str, password: &str) -> Result<usize>;
}

impl<S: Read + Write> IrcWrite for BufStream<S> {
    fn send_raw_message(&mut self, msg: &str) -> Result<usize> {
        let mut message = String::from(msg);
        message = message + "\r\n";

        let write_result = self.write(message.as_bytes());
        let flush_result = self.flush();

        write_result
    }

    fn send_message(&mut self, destination: &str, msg: &str) -> Result<usize> {
        let mut message = String::from("PRIVMSG ");
        message += destination;
        message += " :";
        message += msg;

        self.send_raw_message(&message)
    }

    fn identify(&mut self, ns_name: &str, password: &str) -> Result<usize> {
        let mut message = String::from("identify ");
        message += password;

        self.send_message(ns_name, &message)
    }
}
