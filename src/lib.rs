use std::{
    env,
    process::exit
};

mod try_parse;
use try_parse::*;

/// `Remainder` if you want to take all of the remained args, e.g.
/// Args is `-f 420 69 -r 555..1024 -m "urmom"`:
/// And you call `parse_many`:
/// ```
/// let flag = Flag::<String>::new("-r", "--range");
/// println!("{:?}", parser.parse_many(&flag, NArgs::Remainder));
/// ```
/// Output will be following: `555..1024 -m "urmom"`, simply all of the remained args to the right.
///
/// <===================================================================================================>
///
/// `SmartRemainder` is works like `Remainder`, but it goes from left to right collects arguments until they're parsable,
/// e.g. if args is `-f 420 69 555..1024 "rakivo"` and you call `parse_many`:
/// ```
/// let flag = Flag::<i32>::new("-f", "--flag");
/// println!("{:?}", parser.parse_many(&flag, NArgs::SmartRemainder));
/// ```
/// Output will be following: `[420, 69]`.
///
/// <===================================================================================================>
///
/// `Count` just calls `iter.take(count)`, i.e. takes `count` elements from left to right.
/// e.g. args is `-f 420 69 1024 777 2048 -r 555..1024 -m "urmom"`:
/// ```
/// let flag = Flag::<i32>::new("-f", "--flag");
/// println!("{:?}", parser.parse_many(&flag, NArgs::Count(3)));
/// ```
/// Output will be following: `[420, 69, 1024]`.
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

/// I separated moving and borrowing methods to conveniently create flags in one line, e.g.
/// ```
/// let flag = Flag::<i32>::new("-f", "--flag").mandatory().help("test");
/// ```
/// If these functions did not move the flag into them, you would encounter a CE: error[E0716]: temporary value dropped while borrowed.
impl<T> Flag<T>
where
    T: TryParse
{
    pub fn new(short: &'static str, long: &'static str) -> Self {
        Self {
            short,
            long,
            help: None,
            mandatory: false,
            default: None::<T>,
        }
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
