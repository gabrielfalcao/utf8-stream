# utf8-stream

Lightweight crate that provides Utf8Stream that "streams" over utf8
chunks and derefs to string slices.

## Example

```rust
use charstream::Utf8Stream;

let stream = Utf8Stream::new("red❤️heart");
assert_eq!(
    stream.map(String::from).collect::<Vec<String>>(),
    vec!["r", "e", "d", "❤️", "h", "e", "a", "r", "t"]
);
```
