#[macro_use]
extern crate ansiform;

#[test]
fn escapes_work() {
    assert_eq!(ansi!("\n"), "\n");
    assert_eq!(ansi!("\r"), "\r");
    assert_eq!(ansi!("{}\n"), "{}\n");
}
