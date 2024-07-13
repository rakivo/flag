use crate::Flag;
use crate::TryParse;

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

#[allow(unused)]
impl Parser {
    #[inline]
    pub const fn new() -> Self {
        Self {
        }
    }

    #[inline]
    pub fn parse<T: TryParse>(&mut self, f: &Flag::<T>) -> Option::<T> {
        T::parse(self, f)
    }

    #[inline]
    pub fn passed(&mut self, f: &Flag::<bool>) -> bool {
        bool::parse(self, f).unwrap()
    }
}
