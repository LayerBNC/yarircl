use bufstream::BufStream;
use message::{IrcMessage, NumericReply};
use std::io::*;
use std::net::{ToSocketAddrs, TcpStream};
use std::str::FromStr;
use user::IrcUser;

#[derive(Debug)]
pub struct IrcClient<A: ToSocketAddrs> {
    pub server: A,
    pub user: IrcUser,
    pub connected: bool,
    pub messages: Vec<IrcMessage>,
    pub server_motd: String,
    pub supported_capabilities: Vec<String>,
    pub enabled_capabilities: Vec<String>,
}

impl<A: ToSocketAddrs> IrcClient<A> {
    pub fn new(server: A, user: IrcUser) -> IrcClient<A> {
        let mut client = IrcClient {
            server: server,
            user: user,
            connected: false,
            messages: Vec::new(),
            server_motd: String::new(),
            supported_capabilities: Vec::new(),
            enabled_capabilities: Vec::new(),
        };

        return client;
    }

    pub fn connect(&mut self) -> BufStream<TcpStream> {
        self.server_motd = String::new();
        self.messages.clear();

        let stream = TcpStream::connect(&self.server).unwrap();
        let mut bufstream = BufStream::new(stream);

        bufstream.register_connection(&self.user);

        self.connected = true;
        return bufstream;
    }

    pub fn conn_handler(&mut self, stream: &mut BufStream<TcpStream>) -> bool {
        let mut buffer = String::new();
        if stream.read_line(&mut buffer).unwrap() > 0 {
            let message = match IrcMessage::from_str(&buffer) {
                Ok(x) => x,
                Err(e) => return false,
            };

            match message.command {
                NumericReply::PING => {
                    let reply = &format!("PONG :{reply}", reply = message.params[0]);
                    stream.send_raw_message(reply);
                }
                NumericReply::CAP => {
                    match &message.params[1][..] {
                        "LS" => {
                            let caps: Vec<&str> = message.params[2].split(' ').collect();
                            self.supported_capabilities.extend(caps.iter()
                                .map(|&x| String::from(x))
                                .collect::<Vec<String>>());

                            // TODO: support cap values

                            let mut want = vec!["multi-prefix",
                                                "server-time",
                                                "extended-join",
                                                "znc.in/server-time-iso"];
                            want.retain(|&cap| self.supported_capabilities.contains(&String::from(cap)));

                            stream.send_raw_message(&format!("CAP REQ :{caps}", caps = want.join(" ")));
                        }
                        "ACK" => {
                            self.enabled_capabilities = message.params[2]
                                .split(' ')
                                .collect::<Vec<&str>>()
                                .iter()
                                .map(|&x| String::from(x.trim()))
                                .collect();
                            println!("Enabled capabilities: {}",
                                     self.enabled_capabilities.join(", "));

                            stream.send_raw_message("CAP END");
                        }
                        _ => {}
                    };
                }
                NumericReply::RPL_MOTD => {
                    self.server_motd.push_str(&message.params[1]);
                    self.server_motd.push_str("\r\n");
                }
                NumericReply::RPL_WHOISUSER => {
                    let user = self.handle_whois(&message);
                    println!("{:?}", user);
                }
                _ => {}
            };

            self.messages.push(message);
            return true;
        } else {
            self.connected = false;
            return false;
        }
    }

    fn handle_whois(&self, message: &IrcMessage) -> IrcUser {
        let nick = &message.params[1];
        let user = &message.params[2];
        let host = &message.params[3];
        let real_name = &message.params[5];

        let mut user = IrcUser::new(nick, user);
        user.set_realname(real_name);
        user.set_hostname(host);

        user
    }
}

pub trait IrcWrite {
    fn send_raw_message(&mut self, msg: &str) -> Result<usize>;
    fn join(&mut self, channel: &str) -> Result<usize>;
    fn send_message(&mut self, destination: &str, msg: &str) -> Result<usize>;
    fn register_connection(&mut self, user: &IrcUser) -> Result<usize>;
}

impl<S: Read + Write> IrcWrite for BufStream<S> {
    fn send_raw_message(&mut self, msg: &str) -> Result<usize> {
        let mut message = String::from(msg);
        message = message + "\r\n";
        println!("<< {}", message);

        let write_result = self.write(message.as_bytes());
        let flush_result = self.flush();

        write_result
    }

    fn join(&mut self, channel: &str) -> Result<usize> {
        self.send_raw_message(&format!("JOIN {channel}", channel = channel))
    }

    fn send_message(&mut self, destination: &str, msg: &str) -> Result<usize> {
        let mut message = String::from("PRIVMSG ");
        message += destination;
        message += " :";
        message += msg;

        self.send_raw_message(&message)
    }

    fn register_connection(&mut self, user: &IrcUser) -> Result<usize> {
        let mut bytes_written: usize = 0;
        match self.send_raw_message("CAP LS") {
            Ok(x) => bytes_written += x,
            Err(e) => return Err(e),
        };

        // Only send PASS if we defined it
        if user.nickserv_password.len() > 0 {
            match self.send_raw_message(&format!("PASS {pass}", pass = user.nickserv_password)) {
                Ok(x) => bytes_written += x,
                Err(e) => return Err(e),
            };
        }

        match self.send_raw_message(&format!("NICK {nick}", nick = user.nick)) {
            Ok(x) => bytes_written += x,
            Err(e) => return Err(e),
        };

        match self.send_raw_message(&format!("USER {user} 0 * :{realname}",
                                             user = user.user,
                                             realname = user.real_name)) {
            Ok(x) => bytes_written += x,
            Err(e) => return Err(e),
        };

        Ok(bytes_written)
    }
}
