use std::{process::Command, io::{self, Write}};

fn main() {
    let t = Command::new("bash")
    .arg("-c")
    .arg("say this is a test")
    .output().unwrap().stdout;
    io::stdout().write_all(&t).unwrap();

}