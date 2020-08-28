#[macro_use]
extern crate ansiform;

fn main() {
    let s = format!(
        ansi!("{;red}{}{:#?;green}"),
        "red",
        "no color",
        Some("green"),
    );
    println!("{}", s);
}
