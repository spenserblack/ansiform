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
        _ => panic!("First argument must be a literal"),
    };

    TokenStream::new()
}
