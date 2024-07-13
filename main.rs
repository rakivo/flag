use procs::*;

#[flag("-f", "--flag", mandatory, help("test"), type("u64"))]

// TODO: get rid of using IO operations to pass args to another runtime.
pub fn init() {
    let flags = std::env::args().skip(1).collect::<Vec::<_>>();
    std::fs::write("tmp.flag", flags.join(" ").as_bytes()).unwrap();
}

fn main() {
    init();
    println!("{FLAG}");
}
