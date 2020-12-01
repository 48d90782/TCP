use crate::errors::TCPError;
use crate::protocol::Protocol;
use std::net::Ipv4Addr;

pub struct IPv4Header<'a> {
    // raw_data is raw protocol frame
    raw_data: &'a [u8],
    // version contains version of IP frame (currently only 4th)
    version: u8,
    // Internet header length
    ihl: u8,
    //
    dscp: u8,
    // Explicit Congestion Notification
    ecn: u8,
    // total len of the frame including header and data
    total_len: u16,
    // https://tools.ietf.org/html/rfc6864
    id: u16,
    // flags
    flags: u8,
    //
    fragment_offset: u16,
    //
    ttl: u8,
}

///  3.2 Frame format:
//   If flag IFF_NO_PI is not set each frame format is:
//      Flags [2 bytes]
//      Proto [2 bytes]
//      Raw Protocol(IP, IPv6, etc) frame.
//  https://en.wikipedia.org/wiki/EtherType
impl<'a> IPv4Header<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        IPv4Header {
            raw_data: data,
            version: 0,
            ihl: 0,
            dscp: 0,
            ecn: 0,
            total_len: 0,
            id: 0,
            flags: 0,
            fragment_offset: 0,
            ttl: 0,
        }
    }

    // 0-th octet
    // lower 4 bits
    pub fn version(&mut self) -> u8 {
        self.version = self.raw_data[0] >> 4;
        self.version
    }

    // 0-th octet
    // upper 4 bits
    // The minimum value for this field is 5,[29] which indicates a length of 5 × 32 bits = 160 bits = 20 bytes.
    // As a 4-bit field, the maximum value is 15,
    // this means that the maximum size of the IPv4 header is 15 × 32 bits, or 480 bits = 60 bytes.
    pub fn ihl(&mut self) -> Result<u8, TCPError> {
        self.ihl = self.raw_data[0] & 0b0000_1111;
        if self.ihl < 5 {
            return Err(TCPError::IHLError {
                cause: "less than 20 bytes".into(),
            });
        }
        Ok(self.ihl)
    }

    //       Bits 0-2:  Precedence.
    //       Bit    3:  0 = Normal Delay,      1 = Low Delay.
    //       Bits   4:  0 = Normal Throughput, 1 = High Throughput.
    //       Bits   5:  0 = Normal Relibility, 1 = High Relibility.
    //       Bit  6-7:  Reserved for Future Use.
    //
    //          0     1     2     3     4     5     6     7
    //       +-----+-----+-----+-----+-----+-----+-----+-----+
    //       |                 |     |     |     |     |     |
    //       |   PRECEDENCE    |  D  |  T  |  R  |  0  |  0  |
    //       |                 |     |     |     |     |     |
    //       +-----+-----+-----+-----+-----+-----+-----+-----+

    // Precedence
    //
    // 111 - Network Control
    // 110 - Internetwork Control
    // 101 - CRITIC/ECP
    // 100 - Flash Override
    // 011 - Flash
    // 010 - Immediate
    // 001 - Priority
    // 000 - Routine

    // Differentiated Services Code Point (DSCP)
    // https://en.wikipedia.org/wiki/IPv4#DSCP
    pub fn dscp(&mut self) -> u8 {
        self.dscp = self.raw_data[1] >> 2;
        self.dscp
    }

    //
    pub fn ecn(&mut self) -> u8 {
        self.ecn = self.raw_data[1] & 0b0000_0011;
        self.ecn
    }

    // 2, 3 octets
    pub fn total_len(&mut self) -> u16 {
        self.total_len = u16::from_be_bytes([self.raw_data[2], self.raw_data[3]]);
        self.total_len
    }

    // 4, 5 octets
    pub fn ident(&mut self) -> u16 {
        self.id = u16::from_be_bytes([self.raw_data[4], self.raw_data[5]]);
        self.id
    }

    // Bit 0: reserved, must be zero
    // Bit 1: (DF) 0 = May Fragment,  1 = Don't Fragment.
    // Bit 2: (MF) 0 = Last Fragment, 1 = More Fragments.
    //
    // 0   1   2
    // +---+---+---+
    // |   | D | M |
    // | 0 | F | F |
    // +---+---+---+
    pub fn dont_fragment(&mut self) -> bool {
        self.flags = self.raw_data[6] >> 5;
        (self.flags & 0b010) > 0
    }

    // flags
    pub fn more_fragments(&mut self) -> bool {
        self.flags = self.raw_data[6] >> 5;
        (self.flags & 0b100) > 0
    }

    // octet number 6 and 7
    // we need lower 5 bits from 6-th octet and whole 7th octet
    // total len 13 bits
    pub fn fragment_offset(&mut self) -> u16 {
        let num6 = self.raw_data[6] & 0b0001_1111;
        self.fragment_offset = u16::from_be_bytes([num6, self.raw_data[7]]);
        self.fragment_offset
    }

    pub fn ttl(&mut self) -> u8 {
        self.ttl = u8::from(self.raw_data[8]);
        self.ttl
    }

    // 9-th octet (byte)
    // Protocol number
    // https://en.wikipedia.org/wiki/List_of_IP_protocol_numbers
    pub fn protocol(&self) -> Protocol {
        match self.raw_data[9] {
            1 => Protocol::ICMP,
            2 => Protocol::IGMP,
            17 => Protocol::UDP,
            number => {
                println!("number is: {}", number);
                Protocol::Unknown
            }
        }
    }

    // https://tools.ietf.org/html/rfc1071
    pub fn ip_header_checksum(&mut self) -> u16 {
        u16::from_be_bytes([self.raw_data[10], self.raw_data[11]])
    }

    pub fn source_address_raw(&self) -> &'a [u8] {
        &self.raw_data[12..16]
    }

    pub fn destination_address_raw(&self) -> &'a [u8] {
        &self.raw_data[16..20]
    }

    pub fn source_address(&mut self) -> Ipv4Addr {
        Ipv4Addr::from([
            self.raw_data[12],
            self.raw_data[13],
            self.raw_data[14],
            self.raw_data[15],
        ])
    }

    pub fn destination_address(&mut self) -> Ipv4Addr {
        Ipv4Addr::from([
            self.raw_data[16],
            self.raw_data[17],
            self.raw_data[18],
            self.raw_data[19],
        ])
    }
}
