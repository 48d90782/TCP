mod ipv4;
mod protocol;

use std::process::Command;
use tun_tap::{Iface, Mode};
use std::io;
use crate::ipv4::IPv4Header;

///  3.2 Frame format:
//   If flag IFF_NO_PI is not set each frame format is:
//      Flags [2 bytes]
//      Proto [2 bytes]
//      Raw Protocol(IP, IPv6, etc) frame.
fn main() -> io::Result<()> {
    // https://en.wikipedia.org/wiki/TUN/TAP
    let iface = Iface::new("tun0", Mode::Tun)?;
    println!("Iface name: {}", iface.name());
    let mut buf = [0; 1504]; // 1500 + 4 bytes header

    loop {
        let nbytes = iface.recv(&mut buf)?;
        println!("bytes read: {}", nbytes);

        let header = IPv4Header::new(&buf);
        println!("flags: {:x}", header.flags());


        let protocol = header.protocol();
        println!("Protocol: {}", header.protocol()); // 0x0800 -> IPv4

        println!("data (without 4 bytes header): {:x?}", &buf[4..nbytes]);

        // skip all non IPv4 frames
        if protocol != protocol::Protocol::IPv4 {
            continue;
        }
    }
}

fn cmd(cmd: &str, args: &[&str]) {
    let ecode = Command::new("ip")
        .args(args)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    println!("Command: {}, exit code: {}", cmd, ecode.code().unwrap());
}
