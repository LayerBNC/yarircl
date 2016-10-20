use user::IrcUser;
use std::net::ToSocketAddrs;

#[derive(Debug)]
pub struct IrcClient<A: ToSocketAddrs> {
    pub server: A,
    pub user: IrcUser
}

impl <A: ToSocketAddrs> IrcClient<A> {
    pub fn new(server: A, user: IrcUser) -> IrcClient<A> {
        IrcClient {
            server: server,
            user: user
        }
    }
}
