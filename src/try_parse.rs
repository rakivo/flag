use std::{
    ops::Range,
    str::FromStr,
    path::PathBuf,
    process::exit
};
use crate::{
    Flag,
    Parser
};

pub trait TryParse
where
    Self: Sized
{
    type Ret = Option::<Self>;
    fn parse(_: &mut Parser, f: &Flag::<Self>) -> Option::<Self>;
}

impl TryParse for () {
    #[inline(always)]
    fn parse(_: &mut Parser, _: &Flag::<()>) -> Self::Ret {
        println!("[WARN] Calling `parse()` on `()` type");
        None
    }
}

impl TryParse for String {
    #[inline(always)]
    fn parse(parser: &mut Parser, flag: &Flag::<String>) -> Self::Ret {
        parser.splitted.iter().skip_while(|x| x != &flag.short && x != &flag.long).skip(1).next().cloned()
    }
}

impl TryParse for PathBuf {
    #[inline(always)]
    fn parse(parser: &mut Parser, flag: &Flag::<PathBuf>) -> Self::Ret {
        parser.splitted.iter().skip_while(|x| x != &flag.short && x != &flag.long).skip(1).next().map(PathBuf::from)
    }
}

impl TryParse for bool {
    #[inline(always)]
    fn parse(parser: &mut Parser, flag: &Flag::<bool>) -> Self::Ret {
        Some(parser.splitted.iter().any(|x| x == flag.short || x == flag.long))
    }
}

fn parse<T>(x: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    x.parse::<_>().map_err(|err| {
        eprintln!("ERROR: Failed to convert `{x}` to integer: {err}");
        exit(1)
    }).unwrap()
}

// General implementation for integer types.
impl TryParse for isize {
    #[inline]
    fn parse(parser: &mut Parser, flag: &Flag::<isize>) -> Self::Ret {
        parser.splitted.iter().skip_while(|x| x != &flag.short && x != &flag.long)
            .skip(1)
            .next()
            .map(|x| parse(&x))
    }
}

impl TryParse for Range::<isize> {
    #[inline]
    fn parse(parser: &mut Parser, flag: &Flag::<Range::<isize>>) -> Self::Ret {
        parser.splitted.iter().skip_while(|x| x != &flag.short && x != &flag.long)
            .skip(1)
            .next()
            .map(|x| {
                let splitted = x.split("..").collect::<Vec::<_>>();
                if splitted.len() != 2 {
                    eprintln!("ERROR: Failed to convert `{x}` to range, expected value like: `69..69`");
                    exit(1)
                }
                let start = parse(splitted[0]);
                let end = parse(splitted[1]);
                start..end
            })
    }
}

macro_rules! impl_try_parse {
    ($($t: ty) *) => {
        $(
            impl TryParse for $t {
                #[inline]
                fn parse(parser: &mut Parser, flag: &Flag::<$t>) -> Self::Ret {
                    isize::parse(parser, &Flag::<_> {
                        short: flag.short,
                        long: flag.long,
                        help: flag.help,
                        mandatory: flag.mandatory,
                        default: flag.default.map(|x| x as _),
                        nargs: flag.nargs.to_owned()
                    }).map(|x| x as _)
                }
            }

            impl TryParse for Range::<$t> {
                #[inline]
                fn parse(parser: &mut Parser, flag: &Flag::<Range::<$t>>) -> Self::Ret {
                    Range::<isize>::parse(parser, &Flag::<_> {
                        short: flag.short,
                        long: flag.long,
                        help: flag.help,
                        mandatory: flag.mandatory,
                        default: flag.default.as_ref().map(|x| Range {
                            start: x.start as _,
                            end: x.end as _,
                        }),
                        nargs: flag.nargs.to_owned()
                    }).map(|x| Range {
                        start: x.start as _,
                        end: x.end as _,
                    })
                }
            }
        ) *
    }
}

impl_try_parse! { i8 i16 i32 i128 u8 u16 u32 u64 u128 usize }
