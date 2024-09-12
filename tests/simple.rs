use std::sync::LazyLock;

use regex::Regex;
use retoco::regex;

#[test]
fn simple() {
    regex!(foo = r"foo");
    assert!(foo::is_match("foo"));
}

#[test]
fn nothing() {
    regex!(nothing = r"\P{any}");
    assert!(!nothing::is_match(""));
}

#[test]
fn empty() {
    regex!(empty = r"");
    static EMPTY: LazyLock<Regex> = LazyLock::new(|| Regex::new("").unwrap());

    assert_eq!(empty::is_match(""), EMPTY.is_match(""));
    assert_eq!(empty::is_match("foo"), EMPTY.is_match("foo"));
}

// #[test]
// fn invalid() {
//     regex!(invalid = r"\");
// }
