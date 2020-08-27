#[macro_use]
extern crate yacc;

fn main() {
    let s = format!(yacc!("{;red}{}{:#?;green}"), "red", "no color", Some("green"));
    println!("{}", s);
}
