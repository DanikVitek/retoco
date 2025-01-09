use regex::Regex;
use retoco::regex;

#[test]
fn literal() {
    regex!(foo = r"foo");
    assert!(foo::is_match("foo"));
    assert!(!foo::is_match("bar"));
}

#[test]
fn nothing() {
    regex!(nothing = r"\P{any}");
    assert!(!nothing::is_match(""));
}

#[test]
fn empty() {
    regex!(empty = "");
    let empty = Regex::new("").unwrap();

    assert_eq!(empty::is_match(""), empty.is_match(""));
    assert_eq!(empty::is_match("foo"), empty.is_match("foo"));
}

// #[test]
// fn invalid() {
//     regex!(invalid = r"\");
// }
