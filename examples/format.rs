#[macro_use]
extern crate yacc;

fn main() {
    let none: Option<()> = None;
    let s = format!(format_str!("{;red}{:?;yellow}{:#?;green}"), "red", none, Some("green"));
    println!("{}", s);
}
