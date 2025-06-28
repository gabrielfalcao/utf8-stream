use utf8_stream::{Utf8Stream};

// #[test]
// fn test_push() {
//     let mut stream = Utf8Stream::new("test");
//     assert_eq!(stream.as_str(), "test");
//     stream.push("icycle");
//     assert_eq!(stream.as_str(), "testicycle");
//     assert_eq!(
//         stream.map(String::from).collect::<Vec<String>>(),
//         vec!["t", "e", "s", "t", "i", "c", "y", "c", "l", "e"]
//     );
// }

// #[test]
// fn test_pop() {
//     let mut stream = Utf8Stream::new("test");
//     assert_eq!(stream.as_str(), "test");
//     assert_eq!(stream.pop(), Some("t"));
//     assert_eq!(stream.pop(), Some("s"));
//     assert_eq!(stream.pop(), Some("e"));
// }

// #[test]
// fn test_pop_utf8_short() {
//     let mut stream = Utf8Stream::new("d❤️h");
//     assert_eq!(stream.as_str(), "d❤️h");
//     assert_eq!(stream.pop(), Some("h"));
//     assert_eq!(stream.pop(), Some("❤️"));
//     assert_eq!(stream.pop(), Some("d"));
//     assert_eq!(stream.pop(), None);
// }

// #[test]
// fn test_pop_utf8() {
//     let mut stream = Utf8Stream::new("red❤️heart");
//     assert_eq!(stream.as_str(), "red❤️heart");
//     assert_eq!(
//         stream.as_bytes(),
//         &[114, 101, 100, 226, 157, 164, 239, 184, 143, 104, 101, 97, 114, 116]
//     );
//     assert_eq!(stream.pop(), Some("t"));
//     assert_eq!(stream.pop(), Some("r"));
//     assert_eq!(stream.pop(), Some("a"));
//     assert_eq!(stream.pop(), Some("e"));
//     assert_eq!(stream.pop(), Some("h"));
//     assert_eq!(stream.pop(), Some("❤️"));
//     assert_eq!(stream.pop(), Some("d"));
//     assert_eq!(stream.pop(), Some("e"));
//     assert_eq!(stream.pop(), Some("r"));
//     assert_eq!(stream.pop(), None);
// }

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

#[test]
fn test_rewind() {
    let mut stream = Utf8Stream::new("red❤️heart");

    assert_eq!(stream.get(0), Some("r"));
    assert_eq!(stream.get(3), Some("❤️"));
    assert_eq!(stream.next(), Some("r"));
    assert_eq!(stream.next(), Some("e"));
    assert_eq!(stream.next(), Some("d"));
    assert_eq!(stream.next(), Some("❤️"));
    stream.rewind();
    assert_eq!(stream.get(0), Some("r"));
    assert_eq!(stream.get(3), Some("❤️"));
    assert_eq!(stream.next(), Some("r"));
    assert_eq!(stream.next(), Some("e"));
    assert_eq!(stream.next(), Some("d"));
    assert_eq!(stream.next(), Some("❤️"));
}

// #[test]
// fn test_last_printable() {
//     let stream = Utf8Stream::new("red❤️heart");
//     assert_eq!(stream.last_printable(), Some("t"));
// }
