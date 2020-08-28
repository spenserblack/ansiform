#[macro_use]
extern crate ansiform;

fn main() {
    println!(ansi!("{first;underline,red,italic} to {0;bold,green,italic}"), 100, first=0);
}
