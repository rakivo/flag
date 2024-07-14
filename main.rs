use procs::*;

#[flag("-f", "--flag", mandatory, help("test"), type("u64"))]

fn main() {
    println!("{FLAG}");
}
