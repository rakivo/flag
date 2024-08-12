use std::{path::PathBuf, ops::Range};
use flager::{Flag, Parser, NArgs, new_flag};

// Evaluated at comptime
const I32_FLAG: Flag::<i32> = new_flag!("-f", "--flag")
    .mandatory()
    .help("A mandatory integer flag");

const MULTI_ARGS_RANGE_FLAG: Flag::<Range::<usize>> = new_flag!("-r", "--range")
    .help("Multiple arguments");

fn main() {
    let parser = Parser::new();

    println!("Flag: {value}", value = parser.parse(&I32_FLAG).unwrap());

    ////////////////////////////////////////

    // Evaluated at runtime
    let path_flag: Flag::<PathBuf> = new_flag!("-p", "--path", "default/path".into())
        .help("An optional path flag");

    println!("Path: {path:?}", path = parser.parse_or_default(&path_flag));

    ////////////////////////////////////////

    let multi_args_flag: Flag::<String> = new_flag!("-a", "--args")
        .help("Multiple arguments");

    println!("Arguments: {args:?}", args = parser.parse_many(&multi_args_flag, NArgs::Remainder));

    // ////////////////////////////////////////                         ^ You can read about `NArgs` in `nargs.md`.

    println!("Ranges: {ranges:?}", ranges = parser.parse_many(&MULTI_ARGS_RANGE_FLAG, NArgs::SmartRemainder));
}
