use utf8_stream::Utf8Stream;

#[test]
fn test_iter_from_static_ref_str() {
    let stream = Utf8Stream::from("test");
    assert_eq!(
        stream.map(String::from).collect::<Vec<String>>(),
        vec!["t", "e", "s", "t"]
    );
}

#[test]
fn test_iter_from_string() {
    let stream = Utf8Stream::from("test".to_string());
    assert_eq!(
        stream.map(String::from).collect::<Vec<String>>(),
        vec!["t", "e", "s", "t"]
    );
}

#[test]
fn test_iter_from_string_slice_ref() {
    let stream = Utf8Stream::from(&"test".to_string());
    assert_eq!(
        stream.map(String::from).collect::<Vec<String>>(),
        vec!["t", "e", "s", "t"]
    );
}
