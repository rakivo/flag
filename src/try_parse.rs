use std::env;
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
    fn parse(_: &mut Parser, flag: &Flag::<String>) -> Self::Ret {
        env::args().skip_while(|x| x != &flag.short && x != &flag.long).skip(1).next()
    }
}

impl TryParse for bool {
    #[inline(always)]
    fn parse(_: &mut Parser, flag: &Flag::<bool>) -> Self::Ret {
        Some(env::args().any(|x| x == flag.short || x == flag.long))
    }
}

// General implementation for integer types.
impl TryParse for isize {
    #[inline]
    fn parse(_: &mut Parser, flag: &Flag::<isize>) -> Self::Ret {
        env::args().skip_while(|x| x != &flag.short && x != &flag.long)
            .skip(1)
            .next()
            .map(|x| x.parse::<_>().expect("Failed to parse argument"))
    }
}

macro_rules! impl_try_parse {
    ($($t: ty) *) => {
        $(
            impl TryParse for $t {
                #[inline]
                fn parse(parser: &mut Parser, flag: &Flag::<$t>) -> Self::Ret {
                    isize::parse(parser, &Flag::<isize> {
                        short: flag.short,
                        long: flag.long,
                        help: flag.help,
                        mandatory: flag.mandatory,
                        default: flag.default.map(|x| x as _),
                        nargs: flag.nargs.to_owned()
                    }).map(|x| x as _)
                }
            }
        ) *
    }
}

impl_try_parse! { i8 i16 i32 i128 u8 u16 u32 u64 u128 usize }
