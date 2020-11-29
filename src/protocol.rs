use std::fmt::{Display, Formatter};
use core::fmt;

#[derive(Eq, PartialEq)]
pub enum Protocol {
    IPv4,
    Unknown,
}

impl Display for Protocol {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Protocol::IPv4 => {
                write!(f, "IPv4")
            }
            Protocol::Unknown => {
                write!(f, "Unknown")
            }
        }
    }
}