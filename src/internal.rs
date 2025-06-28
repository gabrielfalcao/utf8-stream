use crate::Utf8Stream;
use std::alloc::Layout;
use std::fmt::Display;
use std::iter::Iterator;

pub fn get_utf8_at_index<'g>(stream: &Utf8Stream, index: usize) -> (&'g str, usize, usize, usize) {
    let length = stream.length;
    let ptr = stream.ptr;
    if length == 0 {
        return ("", index, 0, 0);
    }
    if get_byte_at_index(ptr, index) < 127 {
        return (get_str_slice_of(ptr, index, 1), index, index + 1, 1);
    }

    let requested_index = index;
    let mut index = index;
    let mut max = length - 1;
    let mut delta = 0;
    let mut offset = 0;
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
        let boff = get_byte_at_index(ptr, noff);
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

pub fn new_ptr(size: usize) -> *mut u8 {
    let layout = Layout::array::<u8>(if size == 0 { 1 } else { size }).unwrap();
    let ptr = unsafe {
        let ptr = std::alloc::alloc_zeroed(layout);
        if ptr.is_null() {
            std::alloc::handle_alloc_error(layout);
        }
        ptr
    };
    let ptr = ptr;
    for a in 0..size {
        unsafe {
            ptr.add(a).write(0);
        }
    }
    ptr
}
pub fn grow_ptr(ptr: *mut u8, old_size: usize, new_size: usize) -> *mut u8 {
    let layout = Layout::array::<u8>(old_size).unwrap();
    let new_ptr = unsafe {
        let new_ptr = std::alloc::realloc(ptr, layout, new_size);
        if new_ptr.is_null() {
            std::alloc::handle_alloc_error(layout);
        }
        new_ptr
    };
    new_ptr
}

pub fn shrink_ptr(ptr: *mut u8, old_size: usize, new_size: usize) -> *mut u8 {
    let layout = Layout::array::<u8>(old_size).unwrap();
    let new_ptr = unsafe {
        let new_ptr = std::alloc::realloc(ptr, layout, new_size);
        if new_ptr.is_null() {
            std::alloc::handle_alloc_error(layout);
        }
        new_ptr
    };
    new_ptr
}

pub fn is_not_ascii_byte(byte: u8) -> bool {
    !is_ascii_printable_byte(byte) || byte > 127
}

pub fn is_ascii_printable_byte(byte: u8) -> bool {
    match byte {
        9..13 | 32..126 => true,
        _ => false,
    }
}
pub fn to_slice_ptr_from_display<T: Display>(input: T) -> *mut u8 {
    let bytes = input.to_string().as_bytes().to_vec();

    let ptr = new_ptr(bytes.len());
    let length = bytes.len();
    if length == 0 {
        return ptr;
    }
    for (i, c) in bytes.iter().enumerate() {
        unsafe {
            ptr.add(i).write(*c);
        }
    }
    ptr
}

pub fn get_byte_at_index<'g>(ptr: *mut u8, index: usize) -> u8 {
    let byte = unsafe { ptr.add(index).read() };
    byte
}

pub fn get_byte_slice_of<'g>(ptr: *mut u8, index: usize, count: usize) -> &'g [u8] {
    let bytes = unsafe { std::slice::from_raw_parts(ptr.add(index), count) };
    bytes
}

pub fn get_str_slice_of<'g>(ptr: *mut u8, index: usize, count: usize) -> &'g str {
    if let Ok(string) = std::str::from_utf8(get_byte_slice_of(ptr, index, count)) {
        string
    } else {
        ""
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
            let mut stream = Utf8Stream::new($string);
            let (string, index, offset, count) = get_utf8_at_index(&stream, $index);
            assert_eq!(string, $expected, $expected);
            assert_eq!(index, $index, "index");
            assert_eq!(offset, $offset, "offset");
            assert_eq!(count, $count, "count");
        };
    }
}
