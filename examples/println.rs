#[macro_use]
extern crate yacc;

fn main() {
    println!(yacc!("{first;red} to {0;green}"), 100, first=0);
}
