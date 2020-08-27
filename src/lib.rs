#[macro_use]
extern crate lazy_static;

extern crate proc_macro;

#[macro_use]
extern crate quote;

extern crate regex;

use proc_macro::{
    TokenStream,
    TokenTree::*,
};
use regex::Regex;

macro_rules! color_str {
    ($s:literal, $color:literal $(,)?) => {
        concat!($color, $s, "\u{001b}[0m")
    }
}

#[proc_macro]
pub fn yacc(tokens: TokenStream) -> TokenStream {
    let mut tokens = tokens.into_iter();
    let format_str = match tokens.next() {
        None => return TokenStream::from( quote! { format!() }),
        Some(Literal(literal)) => literal,
        _ => return TokenStream::from(quote!{ compile_error!("First argument must be a literal") }),
    };

    let format_str = format_str.to_string();
    let mut format_str = format_str.chars();
    let format_str: String = match (format_str.next(), format_str.next_back()) {
        (Some('"'), Some('"')) => format_str.collect(),
        _ => return TokenStream::from(quote!{ compile_error!("First argument must be a literal string") }),
    };

    lazy_static! {
        static ref YACC_ARG: Regex = Regex::new(r"\{(?P<format>[:#?A-z0-9\.]*);(?P<color>\w+)\}").unwrap();
    }
    let format_str = YACC_ARG.replace_all(&format_str, |captures: &regex::Captures| {
        let format = captures.name("format").map(|m| m.as_str()).unwrap_or("");
        let color = captures.name("color").map(|m| m.as_str());
        let format_arg = match color {
            None => format!("{{{format}}}", format=format),
            Some("black") => format!(color_str!("{{{format}}}", "\u{001b}[30m"), format=format),
            Some("red") => format!(color_str!("{{{format}}}", "\u{001b}[31m"), format=format),
            Some("green") => format!(color_str!("{{{format}}}", "\u{001b}[32m"), format=format),
            Some("yellow") => format!(color_str!("{{{format}}}", "\u{001b}[33m"), format=format),
            Some("blue") => format!(color_str!("{{{format}}}", "\u{001b}[34m"), format=format),
            Some("magenta") => format!(color_str!("{{{format}}}", "\u{001b}[35m"), format=format),
            Some("cyan") => format!(color_str!("{{{format}}}", "\u{001b}[36m"), format=format),
            Some("white") => format!(color_str!("{{{format}}}", "\u{001b}[37m"), format=format),
            Some(c) => unimplemented!("Color {} is not supported", c),
        };
        format_arg
    });
    let format_str = format_str.to_string();
    let format_str = format_str.as_str();

    let format_str = quote! {
        #format_str
    };

    let format_str = TokenStream::from(format_str);

    let tokens = format_str;
    tokens
}
