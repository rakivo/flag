use procs::*;
use std::{
    env,
    path::PathBuf,
    process::{exit, Command},
    fs::{write, read_to_string, remove_file},
};

const TMP_FLAG_FILE: &str = "tmp.flag";

#[flag("-f", "--flag", mandatory, help("test"), type("u64"))]

// TODO: get rid of using IO operations to pass args to another runtime.
pub fn init() {
    if PathBuf::from(TMP_FLAG_FILE).exists() { return }

    let flags = env::args().skip(1).collect::<Vec::<_>>().join(" ");
    if matches!(read_to_string(TMP_FLAG_FILE), Ok(s) if s == flags) {
        exit(0);
    }

    write(TMP_FLAG_FILE, flags.as_bytes()).unwrap();

    Command::new("sh")
        .arg("-c")
        .args(["cargo", "build"])
        .output()
        .unwrap();

    let out = Command::new("sh")
        .arg("-c")
        .arg(env::current_exe().unwrap())
        .output()
        .unwrap()
        .stdout;

    print!("{out}", out = String::from_utf8_lossy(&out));
    exit(0);
}

pub fn deinit() {
    remove_file(TMP_FLAG_FILE).ok();
}

fn main() {
    init();

    println!("{FLAG}");

    deinit();
}
