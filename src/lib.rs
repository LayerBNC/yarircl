extern crate bufstream;

#[allow(non_camel_case_types, unused_assignments)]
mod message;
#[allow(unused_assignments)]
mod user;
mod client;
mod error;

pub use client::{IrcClient, IrcWrite};
pub use message::{IrcMessage, NumericReply};
pub use std::str::FromStr;
pub use user::{IrcUser, Hostmask};
