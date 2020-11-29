use std::fmt::{Display, Formatter};
use core::fmt;

#[derive(Eq, PartialEq)]
pub enum Protocol {
    ICMP = 1,
    IGMP = 2,
    UDP = 17,
    Unknown,
}

impl Display for Protocol {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Protocol::ICMP => {
                write!(f, "ICMP")
            }
            Protocol::IGMP => {
                write!(f, "IGMP")
            }
            Protocol::UDP => {
                write!(f, "UDP")
            }
            Protocol::Unknown => {
                write!(f, "Unknown")
            }
        }
    }
}