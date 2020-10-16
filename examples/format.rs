#[macro_use]
extern crate ansiform;

fn main() {
    let s = format!(
        ansi!("{;red}{;black,on bright white}{}{:#?;green}"),
        "red",
        "black on bright white",
        "no color",
        Some("green"),
    );
    println!("{}", s);
}
