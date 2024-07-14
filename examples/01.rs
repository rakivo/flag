use std::{path::PathBuf, ops::Range};
use flager::{Flag, Parser, NArgs};

fn main() {
    let parser = Parser::new();

    let flag = Flag::<i32>::new("-f", "--flag")
        .mandatory()
        .default(420)
        .help("A mandatory integer flag");

    println!("Flag: {value}", value = parser.parse(&flag).unwrap());

    ////////////////////////////////////////

    let flag2 = Flag::<PathBuf>::new("-p", "--path")
        .default("default/path".into())
        .help("An optional path flag");

    println!("Path: {path:?}", path = parser.parse_or_default(&flag2));

    ////////////////////////////////////////

    let flag3 = Flag::<String>::new("-a", "--args")
        .help("Multiple arguments");

    println!("Arguments: {args:?}", args = parser.parse_many(&flag3, NArgs::Remainder));

    ////////////////////////////////////////                         ^ You can read about `NArgs` in `nargs.md`.

    let flag4 = Flag::<Range::<usize>>::new("-r", "--range")
        .help("Multiple arguments");

    println!("Ranges: {ranges:?}", ranges = parser.parse_many(&flag4, NArgs::SmartRemainder));
}
