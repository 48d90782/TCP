use core::fmt;
use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq)]
pub enum Protocol {
    ICMP = 1,
    IGMP = 2,
    TCP = 6,
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
            Protocol::TCP => {
                write!(f, "TCP")
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
