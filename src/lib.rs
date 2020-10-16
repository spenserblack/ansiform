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

/// Adds ANSI escape codes to a formatting string, allowing ANSI colors to be set at compile time instead of runtime.
///
/// # Example
///
/// ```rust
/// # use ansiform::ansi;
///
/// println!(ansi!("It {;green,bold}!"), "works");
/// println!(ansi!("{:#?;black,on bright white}!"), "Notes");
/// println!(ansi!("Warning: {:#?;yellow}!"), Some("warning"));
/// ```
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
        static ref ANSI_ARG: Regex =
            Regex::new(r"\{(?P<format>[:#?A-z0-9\.]*);(?P<color>[\w, ]+)\}").unwrap();
    }
    let format_str = ANSI_ARG.replace_all(&format_str, |captures: &regex::Captures| {
        let format = captures.name("format").map(|m| m.as_str()).unwrap_or("");
        let color = captures.name("color").map(|m| m.as_str());
        let format_arg = match color {
            None => format!("{{{format}}}", format = format),
            Some(options) => {
                let options: Vec<_> = options
                    .split(',')
                    .map(|option| match option {
                        // styles
                        "bold" => "1",
                        "faint" => "2",
                        "italic" => "3",
                        "underline" => "4",
                        // foreground colors
                        "black" => "30",
                        "red" => "31",
                        "green" => "32",
                        "yellow" => "33",
                        "blue" => "34",
                        "magenta" => "35",
                        "cyan" => "36",
                        "white" => "37",
                        "bright black" => "90",
                        "gray" => "90",
                        "grey" => "90",
                        "bright red" => "91",
                        "bright green" => "92",
                        "bright yellow" => "93",
                        "bright blue" => "94",
                        "bright magenta" => "95",
                        "bright cyan" => "96",
                        "bright white" => "97",
                        // background colors
                        "on black" => "40",
                        "on red" => "41",
                        "on green" => "42",
                        "on yellow" => "43",
                        "on blue" => "44",
                        "on magenta" => "45",
                        "on cyan" => "46",
                        "on white" => "47",
                        "on bright black" => "100",
                        "on gray" => "100",
                        "on grey" => "100",
                        "on bright red" => "101",
                        "on bright green" => "102",
                        "on bright yellow" => "103",
                        "on bright blue" => "104",
                        "on bright magenta" => "105",
                        "on bright cyan" => "106",
                        "on bright white" => "107",
                        // other
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
