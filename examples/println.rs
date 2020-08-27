#[macro_use]
extern crate acc;

fn main() {
    println!(acc!("{first;red,underline,faint} to {0;bold,green,italic}"), 100, first=0);
}
