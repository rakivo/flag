#![feature(associated_type_defaults)]
pub enum NArgs {
    Count(usize),
    NoneOrOne,
    DontCare,
    AtLeastOne,
    Remainder
}

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
        std::env::args().skip_while(|x| x != &flag.short && x != &flag.long).skip(1).next()
    }
}

impl TryParse for bool {
    #[inline(always)]
    fn parse(_: &mut Parser, flag: &Flag::<bool>) -> Self::Ret {
        Some(std::env::args().any(|x| x == flag.short || x == flag.long))
    }
}

macro_rules! impl_try_parse {
    ($($t: ty) *) => {
        $(
            impl TryParse for $t {
                #[inline]
                fn parse(_: &mut Parser, flag: &Flag::<$t>) -> Self::Ret {
                    std::env::args().skip_while(|x| x != &flag.short && x != &flag.long)
                        .skip(1)
                        .next()
                        .map(|x| x.parse::<$t>().expect("Failed to parse argument"))
                }
            }
        )*
    }
}

impl_try_parse! { i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }

pub struct Flag<T = ()> {
    short: &'static str,
    long: &'static str,
    help: Option::<&'static str>,
    mandatory: bool,
    value: Option::<T>,
    nargs: NArgs
}

impl<T> Flag<T>
where
    T: TryParse
{
    fn new(short: &'static str, long: &'static str) -> Self {
        Self {
            short,
            long,
            help: None,
            mandatory: false,
            value: None::<T>,
            nargs: NArgs::Count(1)
        }
    }

    #[inline(always)]
    pub fn help(&mut self, help: &'static str) -> &mut Self {
        self.help = Some(help); self
    }

    #[inline(always)]
    pub fn mandatory(&mut self, mandatory: bool) -> &mut Self {
        self.mandatory = mandatory; self
    }
}

#[allow(unused)]
mod parser {
    use std::{
        env::Args,
        iter::Peekable
    };
    pub type Iterator = Peekable::<Args>;
}

pub struct Parser {
}

impl Parser {
    #[inline]
    pub fn new() -> Self {
        Self {
        }
    }

    #[inline]
    pub fn parse<T>(&mut self, f: &Flag::<T>) -> Option::<T>
    where
        T: TryParse,
    {
        T::parse(self, f)
    }

    #[inline]
    pub fn passed(&mut self, f: &Flag::<bool>) -> bool {
        bool::parse(self, f).unwrap()
    }
}

fn main() {
    let flag1 = Flag::<i32>::new("-f", "--flag");
    let flag2 = Flag::<String>::new("-r", "--range");

    let mut parser = Parser::new();

    println!("{:?}", parser.parse(&flag1));
    println!("{:?}", parser.parse(&flag2));
}
