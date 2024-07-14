use std::{
    ops::Range,
    str::FromStr,
    path::PathBuf,
    process::exit
};
use crate::{
    Flag,
    NArgs,
    Parser,
};

pub trait TryParse
where
    Self: Sized
{
    fn parse(_: &Parser, f: &Flag::<Self>) -> Option::<Self>;
    fn parse_many(_: &Parser, f: &Flag::<Self>, nargs: NArgs) -> Option::<Vec::<Self>>;
}

#[inline]
fn parse<T>(parser: &Parser, flag: &Flag::<T>) -> Option::<String> {
    parser.splitted.iter().skip_while(|x| x != &flag.short && x != &flag.long).skip(1).next().cloned()
}

fn parse_many<T>(parser: &Parser, flag: &Flag::<T>, nargs: NArgs) -> Option::<Vec::<String>> {
    let mut iter = parser.splitted.iter().cloned().skip_while(|x| x != &flag.short && x != &flag.long).skip(1);
    if let Some(v) = iter.next() {
        let mut ret = vec![v];
        match nargs {
            NArgs::Count(count)   => ret.extend(iter.take(count - 1)),
            NArgs::Remainder      => ret.extend(iter),
            NArgs::SmartRemainder => ret.extend(iter.take_while(|x| !x.starts_with("-")))
        }
        Some(ret)
    } else {
        None
    }
}

impl TryParse for () {
    #[inline(always)]
    fn parse(_: &Parser, _: &Flag::<()>) -> Option::<Self> {
        println!("[WARN] Calling `parse()` on `()` type");
        None
    }

    #[inline(always)]
    fn parse_many(_: &Parser, _: &Flag::<()>, _: NArgs) -> Option::<Vec::<Self>> {
        println!("[WARN] Calling `parse()` on `()` type");
        None
    }
}

impl TryParse for String {
    #[inline(always)]
    fn parse(parser: &Parser, flag: &Flag::<String>) -> Option::<Self> {
        parse(parser, flag)
    }

    #[inline(always)]
    fn parse_many(parser: &Parser, flag: &Flag::<String>, nargs: NArgs) -> Option::<Vec::<Self>> {
        parse_many(parser, flag, nargs)
    }
}

impl TryParse for PathBuf {
    #[inline(always)]
    fn parse(parser: &Parser, flag: &Flag::<PathBuf>) -> Option::<Self> {
        parse(parser, flag).map(PathBuf::from)
    }

    #[inline(always)]
    fn parse_many(parser: &Parser, flag: &Flag::<PathBuf>, nargs: NArgs) -> Option::<Vec::<Self>> {
        parse_many(parser, flag, nargs).map(|v| v.into_iter().map(PathBuf::from).collect())
    }
}


impl TryParse for bool {
    #[inline(always)]
    fn parse(parser: &Parser, flag: &Flag::<bool>) -> Option::<Self> {
        Some(parser.passed(flag))
    }

    #[inline(always)]
    fn parse_many(parser: &Parser, flag: &Flag::<bool>, _: NArgs) -> Option::<Vec::<Self>> {
        Some(vec![parser.passed(flag)])
    }
}

fn parse_int<T>(x: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    x.parse::<_>().map_err(|err| {
        eprintln!("ERROR: Failed to convert `{x}` to integer: {err}");
        exit(1)
    }).unwrap()
}

fn parse_range<T>(x: &str) -> Range::<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    let splitted = x.split("..").collect::<Vec::<_>>();
    if splitted.len() != 2 {
        eprintln!("ERROR: Failed to convert `{x}` to range, expected value like: `69..69`");
        exit(1)
    }
    let start = parse_int(splitted[0]);
    let end = parse_int(splitted[1]);
    start..end
}

impl TryParse for isize {
    #[inline]
    fn parse(parser: &Parser, flag: &Flag::<isize>) -> Option::<Self> {
        parse(parser, flag).map(|x| parse_int(&x))
    }

    #[inline]
    fn parse_many(parser: &Parser, flag: &Flag::<isize>, nargs: NArgs) -> Option::<Vec::<Self>> {
        parse_many(parser, flag, nargs).map(|v| v.into_iter().map(|x| parse_int(&x)).collect())
    }
}

impl TryParse for Range::<isize> {
    #[inline]
    fn parse(parser: &Parser, flag: &Flag::<Range::<isize>>) -> Option::<Self> {
        parse(parser, flag).map(|x| parse_range(&x))
    }

    #[inline]
    fn parse_many(parser: &Parser, flag: &Flag::<Range::<isize>>, nargs: NArgs) -> Option::<Vec::<Self>> {
        parse_many(parser, flag, nargs).map(|v| v.into_iter().map(|x| parse_range(&x)).collect())
    }
}

macro_rules! impl_try_parse {
    ($($t: ty) *) => {
        $(
            impl TryParse for $t {
                #[inline]
                fn parse(parser: &Parser, flag: &Flag::<$t>) -> Option::<Self> {
                    isize::parse(parser, &Flag::<_> {
                        short: flag.short,
                        long: flag.long,
                        help: flag.help,
                        mandatory: flag.mandatory,
                        default: flag.default.map(|x| x as _),
                    }).map(|x| x as _)
                }

                #[inline]
                fn parse_many(parser: &Parser, flag: &Flag::<$t>, nargs: NArgs) -> Option::<Vec::<Self>> {
                    isize::parse_many(parser, &Flag::<_> {
                        short: flag.short,
                        long: flag.long,
                        help: flag.help,
                        mandatory: flag.mandatory,
                        default: flag.default.map(|x| x as _),
                    }, nargs).map(|x| x.into_iter().map(|x| x as _).collect())
                }
            }

            impl TryParse for Range::<$t> {
                #[inline]
                fn parse(parser: &Parser, flag: &Flag::<Range::<$t>>) -> Option::<Self> {
                    Range::<isize>::parse(parser, &Flag::<_> {
                        short: flag.short,
                        long: flag.long,
                        help: flag.help,
                        mandatory: flag.mandatory,
                        default: flag.default.as_ref().map(|x| Range {
                            start: x.start as _,
                            end: x.end as _,
                        }),
                    }).map(|x| Range {
                        start: x.start as _,
                        end: x.end as _,
                    })
                }

                #[inline]
                fn parse_many(parser: &Parser, flag: &Flag::<Range::<$t>>, nargs: NArgs) -> Option::<Vec::<Self>> {
                    Range::<isize>::parse_many(parser, &Flag::<_> {
                        short: flag.short,
                        long: flag.long,
                        help: flag.help,
                        mandatory: flag.mandatory,
                        default: flag.default.as_ref().map(|x| Range {
                            start: x.start as _,
                            end: x.end as _,
                        }),
                    }, nargs).map(|x| x.into_iter().map(|x| Range {
                        start: x.start as _,
                        end: x.end as _,
                    }).collect())
                }
            }
        ) *
    }
}

impl_try_parse! { i8 i16 i32 i128 u8 u16 u32 u64 u128 usize }
