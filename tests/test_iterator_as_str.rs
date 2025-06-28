use utf8_stream::Utf8Stream;

#[test]
fn test_utf8_esoteric() {
    let mut stream = Utf8Stream::new("👩🏽‍🚒");
    assert_eq!(stream.as_str(), "👩🏽‍🚒");
    assert_eq!(stream.get(0), Some("👩🏽‍🚒"));
    assert_eq!(stream.next(), Some("👩🏽‍🚒"));
    assert_eq!(stream.next_back(), Some("👩🏽‍🚒"));
}

#[test]
fn test_iter_new() {
    let mut stream = Utf8Stream::new("test");
    assert_eq!(stream.as_str(), "test");
    assert_eq!(stream.as_bytes(), b"test");
    assert_eq!(stream.next(), Some("t"));
    assert_eq!(stream.next(), Some("e"));
    assert_eq!(stream.next_back(), Some("e"));
    assert_eq!(stream.next_back(), Some("t"));
    assert_eq!(
        stream.map(String::from).collect::<Vec<String>>(),
        vec!["t", "e", "s", "t"]
    );
}

#[test]
fn test_utf8_heart() {
    let stream = Utf8Stream::new("❤️");
    assert_eq!(stream.as_str(), "❤️");
    assert_eq!(
        stream.map(String::from).collect::<Vec<String>>(),
        vec!["❤️"]
    );
}

#[test]
fn test_utf8_redheart() {
    let stream = Utf8Stream::new("red❤️heart");
    assert_eq!(stream.as_str(), "red❤️heart");
    assert_eq!(
        stream.map(String::from).collect::<Vec<String>>(),
        vec!["r", "e", "d", "❤️", "h", "e", "a", "r", "t"]
    );
}

#[test]
fn test_utf8_fire_fighter() {
    let stream = Utf8Stream::new("fire👩🏽‍🚒fighter");
    assert_eq!(stream.as_str(), "fire👩🏽‍🚒fighter");
    assert_eq!(
        stream.map(String::from).collect::<Vec<String>>(),
        vec!["f", "i", "r", "e", "👩🏽‍🚒", "f", "i", "g", "h", "t", "e", "r"]
    );
}
