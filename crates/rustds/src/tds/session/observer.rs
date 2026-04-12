#![allow(clippy::all)]
// use crate::tds::types::prelude::*;
use alloc::string::String;

#[derive(Debug, Clone)]
pub enum Event {
    StateTransition {
        from: &'static str,
        to: &'static str,
    },
    PreLoginSent,
    PreLoginReceived,
    Login7Sent,
    // Login7AckRecieved,
    BytesSent {
        heading: &'static str,
        len: usize,
    },
    BytesReceived {
        heading: &'static str,
        len: usize,
    },
    Log(String),
}

impl core::fmt::Display for Event {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Event::Log(x) => write!(f, "{x}"),
            Event::StateTransition { from, to } => write!(f, "State: {from} -> {to}"),
            Event::PreLoginSent => write!(f, "PreLogin sent"),
            Event::PreLoginReceived => write!(f, "PreLogin received"),
            Event::Login7Sent => write!(f, "Login7 sent — awaiting server response ..."),
            Event::BytesSent { heading, len } => {
                write!(f, "SENT [{heading}] ({len} bytes)")
            }
            Event::BytesReceived { heading, len } => {
                write!(f, "RECV [{heading}] ({len} bytes)")
            }
        }
    }
}
