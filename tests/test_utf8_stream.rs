use utf8_stream::Utf8Stream;

#[test]
fn test_iter_new() {
    let stream = Utf8Stream::new("test");
    assert_eq!(stream.as_str(), "test");
    assert_eq!(stream.as_bytes(), b"test");
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
fn test_utf8_esoteric() {
    let stream = Utf8Stream::new("👩🏽‍🚒");
    assert_eq!(stream.as_str(), "👩🏽‍🚒");
    assert_eq!(
        stream.map(String::from).collect::<Vec<String>>(),
        vec!["👩🏽‍🚒"]
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
#[test]
fn test_push() {
    let mut stream = Utf8Stream::new("test");
    stream.push("icycle");
    assert_eq!(
        stream.map(String::from).collect::<Vec<String>>(),
        vec!["t", "e", "s", "t", "i", "c", "y", "c", "l", "e"]
    );
}

// // #[test]
// // fn test_get_ascii() {
// //     let mut stream = Utf8Stream::new("test");
// //     assert_eq!(stream.get(4), (0, None));
// //     assert_eq!(stream.get(3), (1, Some('t')));
// //     assert_eq!(stream.get(2), (1, Some('s')));
// //     assert_eq!(stream.get(1), (1, Some('e')));
// //     assert_eq!(stream.get(0), (1, Some('t')));
// // }
//
// #[test]
// fn test_utf8() {
//     let mut stream = Utf8Stream::new("❤️💛🩶🖤🤍❤️");
//     assert_eq!(stream.as_bytes(), "❤️💛🩶🖤🤍❤️".as_bytes().to_vec());
//     // assert_eq!(stream.as_str(), "❤️💛🩶🖤🤍❤️");
//
//     // assert_eq!(stream.len(), 28);
//     // assert_eq!(stream.get(26), (1, Some('\u{2764}')));
// }
//
// // #[test]
// // fn test_pop() {
// //     let mut stream = Utf8Stream::new("test");
// //     assert_eq!(stream.pop(), Some('t'));
// //     assert_eq!(stream.pop(), Some('s'));
// //     assert_eq!(stream.pop(), Some('e'));
// //     assert_eq!(stream.map(String::from).collect::<Vec<String>>(), vec!["t"]);
// // }
//
// #[test]
// fn test_as_str() {
//     let stream = Utf8Stream::new("test");
//     assert_eq!(stream.as_str(), "test");
// }
//
// #[test]
// fn test_deref() {
//     let stream = Utf8Stream::new("test");
//     assert_eq!(&*stream, "test");
// }
//
// #[test]
// fn test_contains() {
//     let mut stream = Utf8Stream::new("test");
//     assert_eq!(stream.contains("es"), true);
// }
//
// #[test]
// fn test_clear() {
//     let mut stream = Utf8Stream::new("test");
//
//     assert_eq!(stream.next(), Some('t'),);
//     assert_eq!(stream.next(), Some('e'),);
//     stream.clear();
//     assert_eq!(stream.next(), None,);
//     stream.push('s');
//     assert_eq!(stream.next(), Some('s'),);
// }
//
// #[test]
// fn test_iter_from_str() {
//     let stream = Utf8Stream::from("test");
//     assert_eq!(
//         stream.map(String::from).collect::<Vec<String>>(),
//         vec!["t", "e", "s", "t"]
//     );
// }
//
// #[test]
// fn test_iter_from_string() {
//     let stream = Utf8Stream::from("test".to_string());
//     assert_eq!(
//         stream.map(String::from).collect::<Vec<String>>(),
//         vec!["t", "e", "s", "t"]
//     );
// }
//
// #[test]
// fn test_iter_from_string_ref() {
//     let stream = Utf8Stream::from(&"test".to_string());
//     assert_eq!(
//         stream.map(String::from).collect::<Vec<String>>(),
//         vec!["t", "e", "s", "t"]
//     );
// }
//
// #[test]
// fn test_iter_from_extend_str() {
//     let mut stream = Utf8Stream::default();
//     stream.extend("test".chars());
//     assert_eq!(
//         stream.map(String::from).collect::<Vec<String>>(),
//         vec!["t", "e", "s", "t"]
//     );
// }
//
// #[test]
// fn test_iter_from_extend_bytes() {
//     let mut stream = Utf8Stream::default();
//     stream.extend("test".bytes());
//     assert_eq!(
//         stream.map(String::from).collect::<Vec<String>>(),
//         vec!["t", "e", "s", "t"]
//     );
// }
//
// #[test]
// fn test_iter_from_extend_string_iterator() {
//     let mut stream = Utf8Stream::default();
//     stream.extend(vec![
//         "t".to_string(),
//         "e".to_string(),
//         "s".to_string(),
//         "t".to_string(),
//     ]);
//     assert_eq!(
//         stream.map(String::from).collect::<Vec<String>>(),
//         vec!["t", "e", "s", "t"]
//     );
// }
//
// #[test]
// fn test_iter_from_extend_str_iterator() {
//     let mut stream = Utf8Stream::default();
//     stream.extend(vec!["t", "e", "s", "t"]);
//     assert_eq!(
//         stream.map(String::from).collect::<Vec<String>>(),
//         vec!["t", "e", "s", "t"]
//     );
// }
//
