#![feature(const_mut_refs)]
#![feature(associated_type_defaults)]
extern crate proc_macro;

mod flag;
mod nargs;
mod parser;
mod try_parse;

use flag::*;
use nargs::*;
use parser::*;
use try_parse::*;

use std::sync::LazyLock;
use proc_macro::{
    Span,
    Ident,
    Punct,
    Spacing,
    Literal,
    TokenTree as Tt,
    TokenStream as Ts
};
use syn::{
    Lit,
    Meta,
    AttributeArgs,
    NestedMeta as NM,
    parse_macro_input,
};
use std::env;

const FLAGS_VAR: &str = "FLAGS";

static FLAGS: LazyLock::<String> = LazyLock::new(|| env::var(FLAGS_VAR).expect(&format!("Set flags to {FLAGS_VAR} env. variable")));
static mut PARSER: Parser = Parser::new();

macro_rules! construct_flag {
    ($ty: ty, $s: expr, $l: expr, $h: expr, $m: expr, $n: expr) => {
        Flag::<$ty> {
            short: $s, long: $l, help: $h, mandatory: $m,
            nargs: NArgs::DontCare,
            value: None::<$ty>
        }
    };
}

#[proc_macro_attribute]
pub fn flag(args: Ts, input: Ts) -> Ts {
    let mut iter = parse_macro_input!(args as AttributeArgs).into_iter();

    let (Some(shortt), Some(longt)) = (iter.next(), iter.next()) else {
        panic!("Expected short and long flag, for example: `#[flag(\"-f\", \"--flag\")]`")
    };

    let (NM::Lit(Lit::Str(shortt)), NM::Lit(Lit::Str(longt))) = (shortt, longt) else {
        panic!("Expected short and long flags to be string literals`")
    };

    let short = shortt.value();
    let long = longt.value();

    let mut ty = None;
    let mut help = None;
    let mut mandatory = false;

    for arg in iter {
        match arg {
            NM::Meta(Meta::Path(path)) if path.is_ident("mandatory") => {
                mandatory = true;
            }
            NM::Meta(Meta::List(ml)) if ml.path.is_ident("help") => {
                if let NM::Lit(Lit::Str(h)) = ml.nested.first().expect("Expected argument for help attribute") {
                    help = Some(h.value());
                } else {
                    panic!("Expected help value to be string literal")
                }
            }
            NM::Meta(Meta::List(ml)) if ml.path.is_ident("type") => {
                if let NM::Lit(Lit::Str(typ)) = ml.nested.first().expect("Expected argument for type attribute") {
                    ty = Some(typ.value());
                } else {
                    panic!("Expected type to be string literal")
                }
            }
            _ => {}
        }
    }

    println!("Short flag: {short:?}");
    println!("Long flag: {long:?}");
    println!("Mandatory: {mandatory}");
    println!("Help: {help:?}");
    println!("Type: {ty:?}");

    let ty = ty.unwrap_or("()".to_owned());

    let name = long.chars()
        .skip_while(|x| x == &'-')
        .collect::<String>()
        .to_uppercase();

    let flag = match ty.as_str() {
        "u64" => construct_flag!(u64, short, long, help, mandatory, nargs),
        _ => panic!()
    };

    let ts = if let Some(val) = unsafe { PARSER.parse(&flag) } {
        let mut ts = vec![
            Tt::Ident(Ident::new("const", Span::call_site())),
            Tt::Ident(Ident::new(&name, Span::call_site())),
            Tt::Punct(Punct::new(':', Spacing::Alone)),
            Tt::Ident(Ident::new(&ty, Span::call_site())),
            Tt::Punct(Punct::new('=', Spacing::Alone)),
            Tt::Literal(Literal::usize_unsuffixed(val as usize)),
            Tt::Punct(Punct::new(';', Spacing::Joint)),
        ];
        ts.extend(input);
        Ts::from_iter(ts)
    } else { input };

    ts
}
