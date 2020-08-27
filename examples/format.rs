#[macro_use]
extern crate acc;

fn main() {
    let s = format!(acc!("{;red}{}{:#?;green}"), "red", "no color", Some("green"));
    println!("{}", s);
}
