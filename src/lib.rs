use std::alloc::Layout;
use std::fmt::{Debug, Display, Formatter};
use std::iter::{Extend, FromIterator, IntoIterator, Iterator};
use std::marker::PhantomData;
use std::ops::Deref;

/// Utf8Stream
///
/// ## Example
///
/// ```
/// use utf8_stream::Utf8Stream;
/// let stream = Utf8Stream::new("fire👩🏽‍🚒fighter");
/// assert_eq!(stream.as_str(), "fire👩🏽‍🚒fighter");
/// /// ```
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Utf8Stream<'a> {
    ptr: *mut u8,
    index: usize,
    length: usize,
    _marker: PhantomData<&'a u8>,
}

impl<'a> Default for Utf8Stream<'a> {
    fn default() -> Utf8Stream<'a> {
        Utf8Stream {
            ptr: to_slice_ptr_from_display(""),
            index: 0,
            length: 0,
            _marker: PhantomData,
        }
    }
}
impl<'a> Drop for Utf8Stream<'a> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            let layout = Layout::array::<u8>(self.length).unwrap();
            unsafe {
                std::alloc::dealloc(self.ptr, layout);
            }
        }
    }
}
impl<'a> Utf8Stream<'a> {
    /// Creates a new [Utf8Stream](Self) from any implementor of [`Display`](std::fmt::Display)
    ///
    /// ```
    /// use utf8_stream::Utf8Stream;
    /// let stream = Utf8Stream::new("red❤️heart");
    /// assert_eq!(stream.as_str(), "red❤️heart");
    /// ```
    pub fn new<T: Display>(input: T) -> Utf8Stream<'a> {
        let input = input.to_string();
        let ptr = to_slice_ptr_from_display(&input);
        let length = input.len();
        Utf8Stream {
            index: 0,
            ptr,
            length,
            _marker: PhantomData,
        }
    }

    /// Pushes more string-like data into an [Utf8Stream](Self)
    ///
    /// ```
    /// use utf8_stream::Utf8Stream;
    /// let mut stream = Utf8Stream::new("red❤️");
    ///
    /// stream.push("heart");
    ///
    /// assert_eq!(stream.as_str(), "red❤️heart");
    /// ```
    pub fn push<T: Display>(&mut self, input: T) {
        let new_chars = input.to_string().as_bytes().to_vec();
        let new_chars_length = new_chars.len();
        let old_length = self.length;
        let new_length = old_length + new_chars_length;
        grow_ptr(self.ptr, new_length);
        self.length = new_length;
        for (a, c) in (old_length..new_length).zip(new_chars.iter()) {
            unsafe {
                self.ptr.add(a).write(*c);
            }
        }
    }

    pub fn contains<T: Display>(&mut self, input: T) -> bool {
        self.as_str().contains(&input.to_string())
    }

    pub fn clear(&mut self) {
        shrink_ptr(self.ptr, 1);
        self.length = 0;
        self.index = 0;
    }

    pub fn rewind(&mut self) {
        self.index = 0;
    }

    pub fn len(&self) -> usize {
        self.length
    }
    fn esoteric_utf8_offset(&mut self, index: usize) -> Option<&'a str> {
        let mut max = self.length - index;
        #[allow(unused_assignments)]
        let mut offset_byte = 0;
        let mut delta = 0;
        for offset in 0..max {
            if index + offset >= self.length {
                continue;
            }
            let offset = max - if offset == 0 { 0 } else { offset % max };
            offset_byte = unsafe { self.ptr.add(index + offset - 1).read() };
            if offset_byte < 127 {
                max = offset;
                continue;
            }
            delta += 1;
            let bytes = unsafe { std::slice::from_raw_parts(self.ptr.add(index), offset) };
            match std::str::from_utf8(bytes) {
                Ok(c) => {
                    if offset < max && is_not_ascii_byte(offset_byte) {
                        self.index = index + max - delta + 1;
                        let max_delta = delta - 1;
                        return Some(unsafe {
                            std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                                self.ptr.add(index),
                                max - max_delta,
                            ))
                        });
                    } else {
                    }
                    self.index = index + offset;
                    return Some(c);
                }
                Err(_e) => {}
            }
        }
        None
    }
    pub fn as_str(&self) -> &str {
        let mut offset = self.length;
        loop {
            if let Ok(slice) =
                std::str::from_utf8(unsafe { std::slice::from_raw_parts(self.ptr, offset) })
            {
                break slice;
            }
            if offset > 0 {
                offset -= 1;
            } else {
                break "";
            }
        }
    }
}
impl<'a> Iterator for Utf8Stream<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.length {
            let index = self.index;
            self.index += 1;
            let byte = unsafe { self.ptr.add(index).read() };
            if is_not_ascii_byte(byte) {
                return self.esoteric_utf8_offset(index);
            }
            let bytes = unsafe { std::mem::transmute::<&[u8], &'a [u8]>(&[byte]) };
            if let Ok(c) = std::str::from_utf8(bytes) {
                Some(c)
            } else {
                None
            }
        } else {
            None
        }
    }
}

fn to_slice_ptr_from_display<T: Display>(input: T) -> *mut u8 {
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

impl<'a> Extend<char> for Utf8Stream<'a> {
    fn extend<T: IntoIterator<Item = char>>(&mut self, iter: T) {
        for string in iter {
            self.push(string);
        }
    }
}
impl<'a> Extend<String> for Utf8Stream<'a> {
    fn extend<T: IntoIterator<Item = String>>(&mut self, iter: T) {
        for string in iter {
            self.push(string);
        }
    }
}
impl<'a> Extend<&'a str> for Utf8Stream<'a> {
    fn extend<T: IntoIterator<Item = &'a str>>(&mut self, iter: T) {
        for string in iter {
            self.push(string);
        }
    }
}
impl<'a> Extend<u8> for Utf8Stream<'a> {
    fn extend<T: IntoIterator<Item = u8>>(&mut self, iter: T) {
        for u in iter {
            self.push(char::from(u));
        }
    }
}

impl<'a> FromIterator<String> for Utf8Stream<'a> {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Utf8Stream<'a> {
        let mut buf = Utf8Stream::default();
        buf.extend(iter);
        buf
    }
}
impl<'a> FromIterator<u8> for Utf8Stream<'a> {
    fn from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Utf8Stream<'a> {
        let mut buf = Utf8Stream::default();
        buf.extend(iter);
        buf
    }
}

impl<'a> FromIterator<char> for Utf8Stream<'a> {
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Utf8Stream<'a> {
        let mut buf = Utf8Stream::default();
        buf.extend(iter);
        buf
    }
}

impl<'a> FromIterator<&'a str> for Utf8Stream<'a> {
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Utf8Stream<'a> {
        let mut buf = Utf8Stream::default();
        buf.extend(iter);
        buf
    }
}

impl<'a> From<&str> for Utf8Stream<'a> {
    fn from(s: &str) -> Utf8Stream<'a> {
        Utf8Stream::new(s)
    }
}

impl<'a> From<String> for Utf8Stream<'a> {
    fn from(s: String) -> Utf8Stream<'a> {
        Utf8Stream::new(s)
    }
}

impl<'a> From<&String> for Utf8Stream<'a> {
    fn from(s: &String) -> Utf8Stream<'a> {
        Utf8Stream::new(s)
    }
}

impl<'a> Display for Utf8Stream<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'a> Deref for Utf8Stream<'a> {
    type Target = str;

    fn deref(&self) -> &str {
        self.as_str()
    }
}

fn new_ptr(size: usize) -> *mut u8 {
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
fn grow_ptr(ptr: *mut u8, new_size: usize) -> *mut u8 {
    let new_size = if new_size == 0 { 1 } else { new_size };
    let layout = Layout::array::<u8>(new_size).unwrap();
    let new_ptr = unsafe {
        let new_ptr = std::alloc::realloc(ptr, layout, new_size);
        if new_ptr.is_null() {
            std::alloc::handle_alloc_error(layout);
        }
        new_ptr
    };
    new_ptr
}

fn shrink_ptr(ptr: *mut u8, new_size: usize) -> *mut u8 {
    let new_size = if new_size == 0 { 1 } else { new_size };
    let layout = Layout::array::<u8>(new_size).unwrap();
    let new_ptr = unsafe {
        let new_ptr = std::alloc::realloc(ptr, layout, new_size);
        if new_ptr.is_null() {
            std::alloc::handle_alloc_error(layout);
        }
        new_ptr
    };
    new_ptr
}

fn is_not_ascii_byte(byte: u8) -> bool {
    !is_ascii_printable_byte(byte) || byte > 127
}

fn is_ascii_printable_byte(byte: u8) -> bool {
    match byte {
        9..13 | 32..126 => true,
        _ => false,
    }
}
