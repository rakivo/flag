use std::{path::PathBuf, ops::Range};
use flager::{Flag, Parser, NArgs, new_flag};

const I32_FLAG: Flag::<i32> = new_flag!("-f", "--flag")
    .mandatory()
    .help("A mandatory integer flag");

const MULTI_ARGS_RANGE_FLAG: Flag::<Range::<usize>> = new_flag!("-r", "--range")
    .help("Multiple arguments");

fn main() {
    let parser = Parser::new();

    let path_flag: Flag::<PathBuf> = new_flag!("-p", "--path", "default/path".into())
        .help("An optional path flag");

    let multi_args_flag: Flag::<String> = new_flag!("-a", "--args")
        .help("Multiple arguments");

    println!("Flag: {value}", value = parser.parse(&I32_FLAG).unwrap());
    println!("Path: {path:?}", path = parser.parse_or_default(&path_flag));
    println!("Arguments: {args:?}", args = parser.parse_many(&multi_args_flag, NArgs::Remainder));
    println!("Ranges: {ranges:?}", ranges = parser.parse_many(&MULTI_ARGS_RANGE_FLAG, NArgs::SmartRemainder));
}
