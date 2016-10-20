extern crate irc;

use irc::*;

#[test]
fn test_irc_message_from_str() {
    let raw_str = ":irc.server NOTICE * :*** Looking up your hostname...";
    let generated = Ok(IrcMessage {
        raw: String::from(raw_str),
        prefix: String::from("irc.server"),
        command: NumericReply::NOTICE,
        params: vec![
            String::from("*"), String::from(":*** Looking up your hostname..."),
        ]
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
        nickserv_password: String::from("password")
    };

    let mut new_user = IrcUser::new("nick", "username", "real name");
    new_user.set_password("password");

    println!("{:?}", new_user);

    assert_eq!(correct, new_user);
}
