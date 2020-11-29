use crate::protocol::Protocol;

pub struct IPv4Header<'a> {
    raw_data: &'a [u8],
    version: u8,
    ihl: u8,
}

///  3.2 Frame format:
//   If flag IFF_NO_PI is not set each frame format is:
//      Flags [2 bytes]
//      Proto [2 bytes]
//      Raw Protocol(IP, IPv6, etc) frame.
impl<'a> IPv4Header<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        IPv4Header {
            raw_data: data,
            version: 0,
            ihl: 0,
        }
    }

    // 0-th octet
    // lower 4 bits
    pub fn version(&self) -> u8 {
        let byte = self.raw_data[0];
        byte >> 4
    }

    pub fn ihl(&self) -> u8 {
        let byte = self.raw_data[0];
        byte & 0xF0
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