#[macro_use]
extern crate yacc;

fn main() {
    let string = yacc_format!("{;red}{;yellow}{;green}", "red", "yellow", "green");
    println!("{}", string);
}
