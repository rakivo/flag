use crate::Flag;
use crate::FLAGS;
use crate::Parser;

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

impl TryParse for &'static str {
    #[inline(always)]
    fn parse(_: &mut Parser, flag: &Flag::<&'static str>) -> Self::Ret {
        FLAGS.split_whitespace().skip_while(|x| x != &flag.short && x != &flag.long).skip(1).next()
    }
}

impl TryParse for bool {
    #[inline(always)]
    fn parse(_: &mut Parser, flag: &Flag::<bool>) -> Self::Ret {
        Some(FLAGS.split_whitespace().any(|x| x == flag.short || x == flag.long))
    }
}

macro_rules! impl_try_parse {
    ($($t: ty) *) => {
        $(
            impl TryParse for $t {
                #[inline]
                fn parse(_: &mut Parser, flag: &Flag::<$t>) -> Self::Ret {
                    FLAGS.split_whitespace()
                        .skip_while(|x| x != &flag.short && x != &flag.long)
                        .skip(1)
                        .next()
                        .map(|x| x.parse::<$t>().expect("Failed to parse argument"))
                }
            }
        )*
    }
}

impl_try_parse! { i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }
