use regex::Regex;
use retoco::regex;

#[test]
fn literal() {
    regex!(foo = r"foo");
    assert!(foo::is_match("foo"));
    assert!(foo::is_match("dsdfooaa"));
    assert!(!foo::is_match("bar"));
    assert!(!foo::is_match(""));
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

#[test]
fn class() {
    regex!(digit = r"\d");
    assert!(digit::is_match("1"));
    assert!(!digit::is_match("a"));

    regex!(word = r"\w");
    assert!(word::is_match("a"));
    assert!(word::is_match("1"));
    assert!(!word::is_match(" "));

    regex!(space = r"\s");
    assert!(space::is_match(" "));
    assert!(!space::is_match("a"));

    regex!(any = r"\p{any}");
    assert!(any::is_match("b"));
    assert!(any::is_match("c"));
    assert!(!any::is_match(""));
}

// #[test]
// fn invalid() {
//     regex!(invalid = r"\");
// }
