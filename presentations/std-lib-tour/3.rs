use std::process::Command;

fn main() {
    Command::new("ls")
            .args(&["-l", "-a"])
            .spawn()
            .expect("ls command failed to start");
}