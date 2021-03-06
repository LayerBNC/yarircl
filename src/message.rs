use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum NumericReply {
    RPL_WELCOME,
    RPL_YOURHOST,
    RPL_CREATED,
    RPL_MYINFO,
    RPL_ISUPPORT,
    RPL_WHOISUSER,
    RPL_WHOISSERVER,
    RPL_ENDOFWHOIS,
    RPL_MOTDSTART,
    RPL_MOTD,
    RPL_ENDOFMOTD,
    ERR_NOSUCHNICK,
    PING,
    PRIVMSG,
    NOTICE,
    CAP,
    NONE = -1,
}

impl FromStr for NumericReply {
    type Err = ();

    fn from_str(s: &str) -> Result<NumericReply, ()> {
        match s {
            "PRIVMSG" => Ok(NumericReply::PRIVMSG),
            "NOTICE" => Ok(NumericReply::NOTICE),
            "001" => Ok(NumericReply::RPL_WELCOME),
            "002" => Ok(NumericReply::RPL_YOURHOST),
            "003" => Ok(NumericReply::RPL_CREATED),
            "004" => Ok(NumericReply::RPL_MYINFO),
            "005" => Ok(NumericReply::RPL_ISUPPORT),
            "372" => Ok(NumericReply::RPL_MOTD),
            "311" => Ok(NumericReply::RPL_WHOISUSER),
            "312" => Ok(NumericReply::RPL_WHOISSERVER),
            "318" => Ok(NumericReply::RPL_ENDOFWHOIS),
            "PING" => Ok(NumericReply::PING),
            "CAP" => Ok(NumericReply::CAP),
            _ => Ok(NumericReply::NONE),
        }
    }
}

trait Substring {
    fn substr(&self, start: u32, length: u32) -> &str;
}

impl Substring for str {
    fn substr(&self, start: u32, length: u32) -> &str {
        return &self[start as usize..start as usize + length as usize];
    }
}

#[derive(Debug)]
pub struct IrcMessage {
    pub raw: String,
    pub prefix: String,
    pub command: NumericReply,
    pub params: Vec<String>,
}

impl PartialEq for IrcMessage {
    fn eq(&self, other: &IrcMessage) -> bool {
        self.raw == other.raw
    }
}

// Thanks SirCmpwn for his awesome and simple way to parse IRC messages!
// https://github.com/SirCmpwn/ChatSharp/blob/master/ChatSharp/IrcMessage.cs
impl FromStr for IrcMessage {
    type Err = String;
    fn from_str(s: &str) -> Result<IrcMessage, String> {
        let mut msg = s.trim_right_matches(|c: char| c == '\r' || c == '\n');
        let raw_message_clone = String::from(msg);

        let mut prefix = String::new();
        let mut command: NumericReply = NumericReply::NONE;
        let mut params: Vec<String> = Vec::new();

        if msg.starts_with(":") {
            let whitespace: u32 = match msg.find(' ') {
                Some(x) => x as u32,
                None => 0u32,
            };
            prefix = match String::from_str(msg.substr(1, whitespace - 1)) {
                Ok(x) => x,
                Err(_) => return Err(String::from("error parsing message prefix")),
            };
            msg = msg.substr(whitespace + 1, msg.len() as u32 - (whitespace + 1));
        }

        if msg.contains(' ') {
            let idx = match msg.find(' ') {
                Some(x) => x as u32,
                None => 0u32,
            };
            command = match NumericReply::from_str(msg.substr(0, idx)) {
                Ok(x) => x,
                Err(_) => return Err(format!("error parsing irc message {}", msg)),
            };
            msg = msg.substr(idx + 1, msg.len() as u32 - (idx + 1));

            // Parse parameters
            while msg != "" {
                if msg.starts_with(":") {
                    params.push(String::from(msg.substr(1, msg.len() as u32 - 1)));
                    break;
                }

                if !msg.contains(' ') {
                    params.push(String::from(msg));
                    msg = "";
                    break;
                }

                let idx = match msg.find(' ') {
                    Some(x) => x as u32,
                    None => 0u32,
                };
                params.push(String::from(msg.substr(0, idx)));
                msg = msg.substr(idx + 1, msg.len() as u32 - (idx + 1));
            }
        }

        Ok(IrcMessage {
            raw: raw_message_clone,
            prefix: prefix,
            command: command,
            params: params,
        })
    }
}
