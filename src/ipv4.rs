use crate::protocol::Protocol;

pub struct IPv4Header<'a> {
    raw_data: &'a [u8]
}

impl<'a> IPv4Header<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        IPv4Header {
            raw_data: data
        }
    }

    pub fn flags(&self) -> u16 {
        u16::from_be_bytes([self.raw_data[0], self.raw_data[1]])
    }

    //  https://en.wikipedia.org/wiki/EtherType
    pub fn protocol(&self) -> Protocol {
        match u16::from_be_bytes([self.raw_data[2], self.raw_data[3]]) {
            0x0800 => Protocol::IPv4,
            other => {
                eprintln!("skipping: {}", other);
                Protocol::Unknown
            }
        }
    }
}