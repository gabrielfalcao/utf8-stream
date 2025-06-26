use utf8_stream::Utf8Stream;
#[test]
fn test_push() {
    let mut stream = Utf8Stream::new("test");
    assert_eq!(stream.as_str(), "test");
    stream.push("icycle");
    assert_eq!(stream.as_str(), "testicycle");
    assert_eq!(
        stream.map(String::from).collect::<Vec<String>>(),
        vec!["t", "e", "s", "t", "i", "c", "y", "c", "l", "e"]
    );
}

#[test]
fn test_pop() {
    let mut stream = Utf8Stream::new("test");
    assert_eq!(stream.pop(), Some("t"));
    assert_eq!(stream.pop(), Some("s"));
    assert_eq!(stream.pop(), Some("e"));
    assert_eq!(stream.map(String::from).collect::<Vec<String>>(), vec!["t"]);
}

// #[test]
// fn test_clear() {
//     let mut stream = Utf8Stream::new("test");

//     assert_eq!(stream.next(), Some("t"),);
//     assert_eq!(stream.next(), Some("e"),);
//     stream.clear();
//     assert_eq!(stream.next(), None,);
//     stream.push("s");
//     assert_eq!(stream.next(), Some("s"),);
// }
