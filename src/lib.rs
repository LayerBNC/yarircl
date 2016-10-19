extern crate bufstream;

use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;
pub use std::str::FromStr;
use bufstream::BufStream;

mod message;
pub use message::{IrcMessage, NumericReply};
