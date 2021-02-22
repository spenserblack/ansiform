#[macro_use]
extern crate ansiform;

fn main() {
    let s = format!(
        ansi!("{;red}{;black,on bright white}{}{:#?;inverted,green}"),
        "red",
        "black on bright white",
        "no color",
        Some("inverted green"),
    );
    println!("{}", s);
}
