use error::Error;

#[derive(Debug)]
pub struct IrcUser {
    pub nick: String,
    pub user: String,
    pub real_name: String,
    pub hostname: String,
    pub nickserv_password: String,
}

impl IrcUser {
    pub fn new(nick: &str, user: &str) -> IrcUser {
        IrcUser {
            nick: String::from(nick),
            user: String::from(user),
            real_name: String::new(),
            hostname: String::new(),
            nickserv_password: String::new(),
        }
    }

    pub fn set_password(&mut self, pw: &str) {
        self.nickserv_password = String::from(pw);
    }

    pub fn set_realname(&mut self, real_name: &str) {
        self.real_name = String::from(real_name);
    }

    pub fn set_hostname(&mut self, hostname: &str) {
        self.hostname = String::from(hostname);
    }
}

impl PartialEq for IrcUser {
    fn eq(&self, other: &IrcUser) -> bool {
        self.nick == other.nick && self.user == other.user && self.real_name == other.real_name &&
        self.nickserv_password == other.nickserv_password
    }
}

#[derive(Debug)]
pub struct Hostmask {
    pub nick: String,
    pub user: String,
    pub host: String,
}

impl Hostmask {
    fn new(nick: &str, user: &str, host: &str) -> Hostmask {
        Hostmask {
            nick: String::from(nick),
            user: String::from(user),
            host: String::from(host),
        }
    }
}

use std::str::FromStr;
use std::string::ToString;

impl FromStr for Hostmask {
    type Err = Error;
    fn from_str(s: &str) -> Result<Hostmask, Error> {
        // nick!user@host.domain
        if s.contains("@") {
            let (mut nick, mut user, mut host): (&str, &str, &str) = ("", "", "");

            if s.contains("!") {
                nick = &s[0..s.find("!").unwrap()];
                user = &s[s.find("!").unwrap() + 1..s.find("@").unwrap()];
            } else {
                user = &s[0..s.find("@").unwrap()];
            }
            host = &s[s.find("@").unwrap() + 1..];

            Ok(Hostmask::new(nick, user, host))
        } else {
            Err(Error::InvalidHostmaskString)
        }
    }
}

impl PartialEq for Hostmask {
    fn eq(&self, other: &Hostmask) -> bool {
        self.nick == other.nick && self.user == other.user && self.host == other.host
    }
}

impl ToString for Hostmask {
    fn to_string(&self) -> String {
        format!("{nick}!{user}@{host}",
                nick = self.nick,
                user = self.user,
                host = self.host)
    }
}
