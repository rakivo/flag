//! # Flager: Lightweight command Line Argument Parser
//!
//! #### This is a Rust library that helps you parse command-line arguments in your application. It provides a simple and flexible way to define flags, parse their values, and handle different types of arguments.
//! ## Features:
//! - Flags: You can define flags with short and long names, and specify whether they are mandatory or have a default value.
//! - Parsing: The library can automatically parse the values of the flags and convert them to the desired data type, such as `String`, `PathBuf`, `bool`, and all of the integer types.
//! - Argument Handling: The library provides three different ways to handle the arguments:
//!   - `Remainder`: Collects all the remaining arguments after the flag.
//!   - `SmartRemainder`: Collects the arguments until it encounters a new flag.
//!   - `Count`: Collects a specific number of arguments.
//!
//! ## Usage
//!
//! ### Here's an example of how to use the library:
//!
//! ``` rust
//! use std::{path::PathBuf, ops::Range};
//! use flager::{Flag, Parser, NArgs};
//!
//! fn main() {
//!     let parser = Parser::new();
//!
//!     let flag = Flag::<i32>::new("-f", "--flag")
//!         .mandatory()
//!         .default(420)
//!         .help("A mandatory integer flag");
//!
//!     println!("Flag: {value}", value = parser.parse(&flag).unwrap());
//!
//!     ////////////////////////////////////////
//!
//!     let flag2 = Flag::<PathBuf>::new("-p", "--path")
//!         .default("default/path".into())
//!         .help("An optional path flag");
//!
//!     println!("Path: {path:?}", path = parser.parse_or_default(&flag2));
//!
//!     ////////////////////////////////////////
//!
//!     let flag3 = Flag::<String>::new("-a", "--args")
//!         .help("Multiple arguments");
//!
//!     println!("Arguments: {args:?}", args = parser.parse_many(&flag3, NArgs::SmartRemainder));
//!
//!     ////////////////////////////////////////
//!
//!     let flag4 = Flag::<Range::<usize>>::new("-r", "--range")
//!         .help("Multiple arguments");
//!
//!     println!("Ranges: {ranges:?}", ranges = parser.parse_many(&flag4, NArgs::SmartRemainder));
//! }
//! ```
//! ### In this example, we define three flags:
//! - A mandatory integer flag `-f` or `--flag`.
//! - An optional path flag `-p` or `--path`.
//! - A flag `-a` or `--args` that can accept multiple arguments.
//!
//! #### We then use the Parser to parse the values of these flags and handle the arguments accordingly.
//!
//! ### Supported Types
//! ##### The library supports the following data types:
//! - `String`
//! - `PathBuf`
//! - `bool`
//! - `isize`, `i8`, `i16`, `i32`, `i64`, `i128`
//! - `usize`, `u8`, `u16`, `u32`, `u64`, `u128`
//! - `Range<isize>`, `Range<i8>`, `Range<i16>`, `Range<i32>`, `Range<i64>`, `Range<i128>`
//! - `Range<usize>`, `Range<u8>`, `Range<u16>`, `Range<u32>`, `Range<u64>`, `Range<u128>`
//!
//! #### You can easily add support for other data types by creating a PR to the official repo: <https://github.com/rakivo/flag/>

use std::{
    env,
    process::exit
};

mod try_parse;
use try_parse::*;

/// # `Remainder`
/// #### If you want to take all of the remained args, e.g. args is `-f 420 69 -r 555..1024 -m "urmom"`, and you call `parse_many`:
/// ```
/// let flag = Flag::<String>::new("-r", "--range");
/// println!("{:?}", parser.parse_many(&flag, NArgs::Remainder));
/// ```
/// ### Output will be following: `555..1024 -m "urmom"`, simply all of the remained args to the right.
/// > <===================================================================================================>
/// # `SmartRemainder`
/// #### `SmartRemainder` works like `Remainder`, but it goes from left to right collects arguments until they're parsable, e.g. if args is `-f 420 69 555..1024 "rakivo"` and you call `parse_many`:
/// ```
/// let flag = Flag::<i32>::new("-f", "--flag");
/// println!("{:?}", parser.parse_many(&flag, NArgs::SmartRemainder));
/// ```
/// ### Output will be following: `[420, 69]`.
/// > <===================================================================================================>
/// # `Count`
/// #### `Count` just calls `iter.take(count)`, i.e. takes `count` elements from left to right. e.g. args is `-f 420 69 1024 777 2048 -r 555..1024 -m "urmom"`:
/// ```
/// let flag = Flag::<i32>::new("-f", "--flag");
/// println!("{:?}", parser.parse_many(&flag, NArgs::Count(3)));
/// ```
/// ### Output will be following: `[420, 69, 1024]`.
///
#[derive(Clone)]
pub enum NArgs {
    Remainder,
    Count(usize),
    SmartRemainder,
}

pub struct Flag<T = ()> {
    short: &'static str,
    long: &'static str,
    help: Option::<&'static str>,
    mandatory: bool,
    default: Option::<T>
}

#[macro_export]
macro_rules! new_flag {
    ($short: literal, $long: literal) => {
        Flag::new($short, $long, None)
    };
    ($short: literal, $long: literal, $def: expr) => {
        Flag::new($short, $long, Option::Some($def))
    };
}

/// I separated moving and borrowing methods to conveniently create flags in one line, e.g.
/// ```
/// let flag = Flag::<i32>::new("-f", "--flag").mandatory().help("test");
/// ```
/// If these functions did not move the flag into them, you would encounter a CE: error[E0716]: temporary value dropped while borrowed.
impl<T> Flag<T>
where
    T: TryParse
{
    pub const fn new(short: &'static str, long: &'static str, default: Option::<T>) -> Self {
        Self {
            short,
            long,
            help: None,
            mandatory: false,
            default,
        }
    }

    #[inline(always)]
    pub const fn help(mut self, help: &'static str) -> Self {
        self.help = Some(help); self
    }

    #[inline(always)]
    pub fn help_borrow(&mut self, help: &'static str) -> &mut Self {
        self.help = Some(help); self
    }

    #[inline(always)]
    pub const fn mandatory(mut self) -> Self {
        self.mandatory = true; self
    }

    #[inline(always)]
    pub fn mandatory_borrow(&mut self) -> &mut Self {
        self.mandatory = true; self
    }

    #[inline(always)]
    pub fn default(mut self, v: T) -> Self {
        self.default = Some(v); self
    }

    #[inline(always)]
    pub fn default_borrow(&mut self, v: T) -> &mut Self {
        self.default = Some(v); self
    }
}

pub struct Parser {
    #[allow(unused)]
    string: String,
    splitted: Vec::<String>,
}

impl Parser {
    #[inline]
    pub fn new() -> Self {
        let splitted = env::args().collect::<Vec::<_>>();
        Self {
            string: splitted.join(" "),
            splitted
        }
    }

    /// Panics when flag is mandatory and it's not provided.
    #[inline]
    fn mandatory_check<T>(&self, f: &Flag::<T>) {
        if f.mandatory && !self.passed(f) {
            eprintln!("ERROR: neither `{s}` nor `{l}` flag is passed", s = f.short, l = f.long);
            exit(1)
        }
    }

    /// Tries to find specific flag in `std::env::args()` and convert it to T
    ///
    /// Panics when flag is mandatory and it's not provided or failed to convert passed argument to T.
    #[inline]
    pub fn parse<T>(&self, f: &Flag::<T>) -> Option::<T>
    where
        T: TryParse
    {
        self.mandatory_check(f);
        T::parse(self, f)
    }

    #[inline]
    pub fn parse_many<T>(&self, f: &Flag::<T>, nargs: NArgs) -> Option::<Vec::<T>>
    where
        T: TryParse
    {
        self.mandatory_check(f);
        T::parse_many(self, f, nargs)
    }

    /// Returns result of `T::parse()` or flag.default in case if `T::parse()` failed.
    /// If flag.default is None, returns T::default.
    ///
    /// Panics when flag is mandatory and it's not provided.
    #[inline]
    pub fn parse_or_default<T>(&self, f: &Flag::<T>) -> T
    where
        T: TryParse + Default + Clone
    {
        self.mandatory_check(f);
        T::parse(self, f).unwrap_or(f.default.to_owned().unwrap_or_default())
    }

    /// Checks if specific flag is provided
    #[inline]
    pub fn passed<T>(&self, f: &Flag::<T>) -> bool {
        self.splitted.iter().any(|x| x == f.short || x == f.long)
    }
}
