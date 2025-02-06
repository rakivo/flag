# Flager: Lightweight command Line Argument Parser

#### Crates.io: <https://crates.io/crates/flager>
#### Docs.rs: <https://docs.rs/flager/0.0.2/flager/>

#### This is a Rust library that helps you parse command-line arguments in your application. It provides a simple and flexible way to define flags, parse their values, and handle different types of arguments.
## Features:
- Flags: You can define flags with short and long names, and specify whether they are mandatory or have a default value.
- Parsing: The library can automatically parse the values of the flags and convert them to the desired data type, such as `String`, `PathBuf`, `bool`, and all of the integer types.
- Argument Handling: The library provides three different ways to handle the arguments:
  - `Remainder`: Collects all the remaining arguments after the flag.
  - `SmartRemainder`: Collects the arguments until it encounters a new flag.
  - `Count`: Collects a specific number of arguments.

## Usage

### Here's an example from examples/01.rs of how to use the library:

``` rust
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
```

### In this example, we define three flags:
- A mandatory integer flag `-f` or `--flag`.
- An optional path flag `-p` or `--path`.
- A flag `-a` or `--args` that can accept multiple arguments.

#### We then use the Parser to parse the values of these flags and handle the arguments accordingly.

### Supported Types
##### The library supports the following data types:
- `String`
- `PathBuf`
- `bool`
- `isize`, `i8`, `i16`, `i32`, `i64`, `i128`
- `usize`, `u8`, `u16`, `u32`, `u64`, `u128`
- `Range<isize>`, `Range<i8>`, `Range<i16>`, `Range<i32>`, `Range<i64>`, `Range<i128>`
- `Range<usize>`, `Range<u8>`, `Range<u16>`, `Range<u32>`, `Range<u64>`, `Range<u128>`

#### You can easily add support for other data types by creating a PR to the official repo: <https://github.com/rakivo/flag/>
