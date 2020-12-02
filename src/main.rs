mod errors;
mod ipv4;
mod protocol;

use crate::ipv4::IPv4Header;
use std::io;
use tun_tap::{Iface, Mode};

fn main() -> io::Result<()> {
    // https://en.wikipedia.org/wiki/TUN/TAP
    let iface = Iface::new("tun0", Mode::Tun)?;
    println!("Iface name: {}", iface.name());
    let mut buf = [0; 1504]; // 1500 + 4 bytes header

    loop {
        let _ = iface.recv(&mut buf)?;

        let mut header = IPv4Header::new(&buf[4..]);
        println!(
            "header checksum: {:x}, calculated: {:x}",
            header.ip_header_checksum(),
            header.calculate_checksum()
        );

        // println!(
        //     "IP version: {}, IHL: {}, DSCP: {}, ECN: {}, bytes: {}, Ident: {:x}, DF: {}, F: {}, Fragment offset: {}, TTL: {}, Protocol: {}, Checksum: {:x}, {} -> {}",
        //     header.version(),
        //     header.ihl()?,
        //     header.dscp(),
        //     header.ecn(),
        //     header.total_len(),
        //     header.ident(),
        //     header.dont_fragment(),
        //     header.more_fragments(),
        //     header.fragment_offset(),
        //     header.ttl(),
        //     header.protocol(),
        //     header.ip_header_checksum(),
        //     header.source_address(),
        //     header.destination_address()
        // );
    }
}
