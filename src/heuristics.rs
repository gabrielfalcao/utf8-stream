use crate::Utf8Stream;

use crate::{
    get_byte_at_index, get_byte_slice_of, get_str_slice_of,
    is_not_ascii_byte,
};
/// heuristic function to retrieve human-friendly utf8 strings from a
/// [Utf8Stream's](crate::Utf8Stream) internal data.
pub fn get_utf8_at_index<'g>(stream: &Utf8Stream, index: usize) -> (&'g str, usize, usize, usize) {
    let length = stream.length;
    let ptr = stream.ptr;
    if length == 0 {
        return ("", index, 0, 0);
    }
    if get_byte_at_index(ptr, index) < 127 {
        return (get_str_slice_of(ptr, index, 1), index, 1, 1);
    }

    let requested_index = index;
    let mut index = index;
    let max = length - 1;
    let offset = 0;
    let mut offset = max - if offset == 0 { 0 } else { offset % max };

    index = length - offset;

    while index > 0 && get_byte_at_index(ptr, index) < 127 {
        let nidx = index - 1;
        index = nidx;
    }

    while index < offset && get_byte_at_index(ptr, index) < 127 {
        let nidx = index + 1;
        index = nidx;
    }

    while offset > index && get_byte_at_index(ptr, offset) < 127 {
        let noff = offset - 1;
        offset = noff;
    }
    while index > 0
        && is_not_ascii_byte(get_byte_at_index(ptr, index - 1))
        && std::str::from_utf8(get_byte_slice_of(ptr, index - 1, offset)).is_err()
    {
        index -= 1;
    }
    while offset < length
        && is_not_ascii_byte(get_byte_at_index(ptr, offset + 1))
        && std::str::from_utf8(get_byte_slice_of(ptr, index, offset + 1)).is_ok()
    {
        offset += 1;
    }
    let mut count = 0;
    for index in index..offset + 1 {
        if is_not_ascii_byte(get_byte_at_index(ptr, index)) {
            count += 1;
        }
    }

    if get_byte_at_index(ptr, count) == 0 {
        count -= 1;
    };
    let bytes = get_byte_slice_of(
        ptr,
        index,
        if get_byte_at_index(ptr, offset) == 0 {
            get_byte_slice_of(ptr, index, offset);
            if offset == length && get_byte_at_index(ptr, count) == 0 {
                count
            } else {
                count - 1
            }
        } else {
            count
        },
    );
    if let Ok(c) = std::str::from_utf8(bytes) {
        (c, index, offset, count)
    } else {
        ("", index, offset, count)
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_get_utf8_at_index, get_utf8_at_index, Utf8Stream};

    #[test]
    fn test_pop_utf8_esoteric() {
        assert_get_utf8_at_index!("👩🏽‍🚒", "👩🏽‍🚒", 0, 15, 16);
    }

    #[test]
    fn test_pop_utf8_single() {
        assert_get_utf8_at_index!("❤️", "❤️", 0, 6, 6);
    }

    #[test]
    fn test_pop_utf8_short() {
        assert_get_utf8_at_index!("d❤️h", "❤️", 1, 6, 6);
    }

    #[test]
    fn test_pop_utf8_long() {
        assert_get_utf8_at_index!("red❤️heart", "❤️", 3, 8, 6);
    }

    #[macro_export]
    macro_rules! assert_get_utf8_at_index {
        ($string:literal, $expected:literal, $index:literal, $offset:literal, $count:literal) => {
            let stream = Utf8Stream::new($string);
            let (string, index, offset, count) = get_utf8_at_index(&stream, $index);
            assert_eq!(string, $expected, $expected);
            assert_eq!(index, $index, "index");
            assert_eq!(offset, $offset, "offset");
            assert_eq!(count, $count, "count");
        };
    }
}
