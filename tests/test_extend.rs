use utf8_stream::Utf8Stream;

#[test]
fn test_iter_from_extend_str() {
    let mut stream = Utf8Stream::default();
    stream.extend("test".chars());
    assert_eq!(
        stream.map(String::from).collect::<Vec<String>>(),
        vec!["t", "e", "s", "t"]
    );
}

#[test]
fn test_iter_from_extend_bytes() {
    let mut stream = Utf8Stream::default();
    stream.extend("test".bytes());
    assert_eq!(
        stream.map(String::from).collect::<Vec<String>>(),
        vec!["t", "e", "s", "t"]
    );
}

#[test]
fn test_iter_from_extend_string_iterator() {
    let mut stream = Utf8Stream::default();
    stream.extend(vec![
        "t".to_string(),
        "e".to_string(),
        "s".to_string(),
        "t".to_string(),
    ]);
    assert_eq!(
        stream.map(String::from).collect::<Vec<String>>(),
        vec!["t", "e", "s", "t"]
    );
}

#[test]
fn test_iter_from_extend_str_iterator() {
    let mut stream = Utf8Stream::default();
    stream.extend(vec!["t", "e", "s", "t"]);
    assert_eq!(
        stream.map(String::from).collect::<Vec<String>>(),
        vec!["t", "e", "s", "t"]
    );
}
