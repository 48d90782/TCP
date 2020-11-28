use std::process::Command;

fn main() {

}

fn cmd(cmd: &str, args: &[&str]) {
    let ecode = Command::new("ip")
        .arg(args)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    assert!(ecode.code(), "Failed to execute {}", cmd);
}
