#![feature(associated_type_defaults)]
use std::{
    ops::Range,
};

mod try_parse;
use try_parse::*;

#[derive(Clone)]
pub enum NArgs {
    Count(usize),
    NoneOrOne,
    DontCare,
    AtLeastOne,
    Remainder
}

pub struct Flag<T = ()> {
    short: &'static str,
    long: &'static str,
    help: Option::<&'static str>,
    mandatory: bool,
    default: Option::<T>,
    nargs: NArgs
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
    fn new(short: &'static str, long: &'static str) -> Self {
        Self {
            short,
            long,
            help: None,
            mandatory: false,
            default: None::<T>,
            nargs: NArgs::Count(1)
        }
    }

    #[inline(always)]
    pub fn parse(&mut self) -> Option::<T> {
        T::parse(&mut Parser::new(), self)
    }

    #[inline(always)]
    pub fn help(mut self, help: &'static str) -> Self {
        self.help = Some(help); self
    }

    #[inline(always)]
    pub fn help_borrow(&mut self, help: &'static str) -> &mut Self {
        self.help = Some(help); self
    }

    #[inline(always)]
    pub fn mandatory(mut self) -> Self {
        self.mandatory = true; self
    }

    #[inline(always)]
    pub fn mandatory_borrow(&mut self) -> &mut Self {
        self.mandatory = true; self
    }

    #[inline(always)]
    pub fn default_borrow(&mut self, v: T) -> &mut Self {
        self.default = Some(v); self
    }

    #[inline(always)]
    pub fn default(mut self, v: T) -> Self {
        self.default = Some(v); self
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
    pub fn parse<T: TryParse>(&mut self, f: &Flag::<T>) -> Option::<T> {
        T::parse(self, f)
    }

    #[inline]
    pub fn parse_or_default<T>(&mut self, f: &Flag::<T>) -> T
    where
        T: TryParse + Default + Clone
    {
        T::parse(self, f).unwrap_or(f.default.to_owned().unwrap_or_default())
    }

    #[inline]
    pub fn passed(&mut self, f: &Flag::<bool>) -> bool {
        bool::parse(self, f).unwrap()
    }
}

fn main() {
    let flag1 = Flag::<i32>::new("-f", "--flag").mandatory().help("test").default(34);
    let flag2 = Flag::<Range::<usize>>::new("-r", "--range");
    let flag3 = Flag::<String>::new("-m", "--mom").mandatory();

    let mut parser = Parser::new();

    println!("{:?}", parser.parse(&flag1));
    println!("{:?}", parser.parse(&flag2));
    println!("{:?}", parser.parse_or_default(&flag3));
}