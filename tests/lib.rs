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
