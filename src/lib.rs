#[macro_use]
extern crate proc_macro;

#[macro_use]
extern crate quote;

use proc_macro::{
    TokenStream,
    TokenTree::*,
};

#[proc_macro]
pub fn yacc_format(tokens: TokenStream) -> TokenStream {
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

    let tokens = quote! {
        compile_error!("`yacc_format!` is not implemented");
    };

    TokenStream::from(tokens)
}
