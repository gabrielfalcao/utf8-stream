use utf8_stream::Utf8Stream;

#[test]
fn test_get_ascii() {
    let stream = Utf8Stream::new("test");
    assert_eq!(stream.get(1), Some("e"));
    assert_eq!(stream.get(0), Some("t"));
    assert_eq!(stream.get(2), Some("s"));
    assert_eq!(stream.get(3), Some("t"));
    assert_eq!(stream.get(4), None);
}
