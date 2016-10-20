#[derive(Debug)]
pub struct IrcUser {
    pub nick: String,
    pub user: String,
    pub real_name: String,
    pub hostname: String,
    pub nickserv_password: String
}

impl IrcUser {
    pub fn new(nick: &str, user: &str, real_name: &str) -> IrcUser {
        IrcUser {
            nick: String::from(nick),
            user: String::from(user),
            real_name: String::from(real_name),
            hostname: String::new(),
            nickserv_password: String::new()
        }
    }

    pub fn set_password(&mut self, pw: &str) {
        self.nickserv_password = String::from(pw);
    }
    
}

impl PartialEq for IrcUser {
    fn eq(&self, other: &IrcUser) -> bool {
        self.nick == other.nick &&
        self.user == other.user &&
        self.real_name == other.real_name &&
        self.nickserv_password == other.nickserv_password
    }
}
