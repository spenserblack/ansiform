use std::fmt;
use yacc::format_str;

fn main() {
    let none: Option<()> = None;
    println!(format_str!("{;red}{:?;yellow}{:#?;green}"), "red", none, Some("green"));
    // println!(format_str!("{;red}{:?yellow}{:#?green}"), "red", None, Some("green"));
    // println!(format_str!("{;red}"), "red");
}
