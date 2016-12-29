extern crate irc;

use irc::*;

#[test]
fn test_irc_message_from_str() {
    let raw_str = ":irc.server NOTICE * :*** Looking up your hostname...";
    let generated = Ok(IrcMessage {
        raw: String::from(raw_str),
        prefix: String::from("irc.server"),
        command: NumericReply::NOTICE,
        params: vec![String::from("*"), String::from(":*** Looking up your hostname...")],
    });
    let correct = IrcMessage::from_str(raw_str);

    assert_eq!(correct, generated);
}

#[test]
fn test_irc_user_instantiate() {
    let correct = IrcUser {
        nick: String::from("nick"),
        user: String::from("username"),
        real_name: String::from("real name"),
        hostname: String::new(),
        nickserv_password: String::from("password"),
    };

    let mut new_user = IrcUser::new("nick", "username");
    new_user.set_realname("real name");
    new_user.set_password("password");

    println!("{:?}", new_user);

    assert_eq!(correct, new_user);
}

#[test]
fn test_hostmask_from_str() {
    let correct = Hostmask {
        nick: String::from("Xinayder"),
        user: String::from("~alex"),
        host: String::from("unaffiliated/rockytv"),
    };

    let generated = Hostmask::from_str("Xinayder!~alex@unaffiliated/rockytv");

    assert_eq!(correct, generated.unwrap());
}

#[test]
#[should_panic]
fn test_hostmask_from_invaid_str() {
    let correct = Hostmask {
        nick: String::from("Xinayder"),
        user: String::from("~alex"),
        host: String::from("unaffiliated/rockytv"),
    };

    let generated = Hostmask::from_str("~alex@unaffiliated/rockytv");

    assert_eq!(correct, generated.unwrap());
}

#[test]
fn test_hostmask_to_string() {
    let hostmask = Hostmask {
        nick: String::from("Xinayder"),
        user: String::from("~alex"),
        host: String::from("unaffiliated/rockytv"),
    };

    let correct = String::from("Xinayder!~alex@unaffiliated/rockytv");

    assert_eq!(correct, hostmask.to_string());
}
