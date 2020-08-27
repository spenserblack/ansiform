#[macro_use]
extern crate yacc;

fn main() {
    println!(yacc!("{first;red,underline,faint} to {0;bold,green,italic}"), 100, first=0);
}
