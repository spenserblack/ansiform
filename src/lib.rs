#[cfg(doctest)]
#[macro_use]
extern crate doc_comment;

#[macro_use]
extern crate lazy_static;

extern crate proc_macro;

#[macro_use]
extern crate quote;

extern crate regex;

use proc_macro::{TokenStream, TokenTree::*};
use regex::Regex;

#[proc_macro]
pub fn ansi(tokens: TokenStream) -> TokenStream {
    let mut tokens = tokens.into_iter();
    let format_str = match tokens.next() {
        None => return TokenStream::from(quote! { format!() }),
        Some(Literal(literal)) => literal,
        _ => {
            return TokenStream::from(quote! { compile_error!("First argument must be a literal") })
        }
    };

    let format_str = format_str.to_string();
    let mut format_str = format_str.chars();
    let format_str: String = match (format_str.next(), format_str.next_back()) {
        (Some('"'), Some('"')) => format_str.collect(),
        _ => {
            return TokenStream::from(
                quote! { compile_error!("First argument must be a literal string") },
            )
        }
    };

    lazy_static! {
        static ref YACC_ARG: Regex =
            Regex::new(r"\{(?P<format>[:#?A-z0-9\.]*);(?P<color>[\w,]+)\}").unwrap();
    }
    let format_str = YACC_ARG.replace_all(&format_str, |captures: &regex::Captures| {
        let format = captures.name("format").map(|m| m.as_str()).unwrap_or("");
        let color = captures.name("color").map(|m| m.as_str());
        let format_arg = match color {
            None => format!("{{{format}}}", format = format),
            Some(options) => {
                let options: Vec<_> = options
                    .split(',')
                    .map(|option| match option {
                        "bold" => "1",
                        "faint" => "2",
                        "italic" => "3",
                        "underline" => "4",
                        "black" => "30",
                        "red" => "31",
                        "green" => "32",
                        "yellow" => "33",
                        "blue" => "34",
                        "magenta" => "35",
                        "cyan" => "36",
                        "white" => "37",
                        s => unimplemented!("Not supported: {}", s),
                    })
                    .collect();
                let options: String = options.join(";");
                format!(
                    "\u{001b}[{colors}m{{{format}}}\u{001b}[0m",
                    colors = options,
                    format = format,
                )
            }
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

#[cfg(doctest)]
doctest!("../README.md");
