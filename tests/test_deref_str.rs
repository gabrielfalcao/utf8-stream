use utf8_stream::Utf8Stream;

#[test]
fn test_deref() {
    let stream = Utf8Stream::new("test");
    assert_eq!(&*stream, "test");
}

#[test]
fn test_contains() {
    let mut stream = Utf8Stream::new("test");
    assert_eq!(stream.contains("es"), true);
}
