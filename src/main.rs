use std::process::Command;
use tun_tap::{Iface, Mode};
use std::io;

///  3.2 Frame format:
//   If flag IFF_NO_PI is not set each frame format is:
//      Flags [2 bytes]
//      Proto [2 bytes]
//      Raw protocol(IP, IPv6, etc) frame.
fn main() -> io::Result<()> {
    // https://en.wikipedia.org/wiki/TUN/TAP
    let iface = Iface::new("tun0", Mode::Tun)?;
    println!("Iface name: {}", iface.name());
    let mut buf = [0; 1504]; // 1500 + 4 bytes header

    loop {
        let nbytes = iface.recv(&mut buf)?;
        println!("bytes read: {}", nbytes);


        let flags = u16::from_be_bytes([buf[0], buf[1]]);
        println!("flags: {}", flags);
        let proto = u16::from_be_bytes([buf[2], buf[3]]);
        println!("proto: {}", proto);

        println!("data: {:?}", &buf[..=nbytes]);
    }

    Ok(())
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
