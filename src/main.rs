use std::process::Command;
use tun_tap::{Iface, Mode};
use std::io;

fn main() -> io::Result<()> {
    let iface = Iface::new("tun0", Mode::Tun)?;
    println!("Iface name: {}", iface.name());
    let mut buf = [0; 1504]; // 1500 + 4 bytes header

    loop {
        let nbytes = iface.recv(&mut buf)?;
        println!("bytes read: {}", nbytes);
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
