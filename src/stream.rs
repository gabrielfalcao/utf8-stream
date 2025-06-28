use std::alloc::Layout;
use std::fmt::{Debug, Display, Formatter};
use std::iter::{Extend, FromIterator, IntoIterator, Iterator};
use std::marker::PhantomData;
use std::ops::Deref;

use crate::internal::{
    get_byte_slice_of, get_utf8_at_index, grow_ptr, is_not_ascii_byte, shrink_ptr,
    to_slice_ptr_from_display,
};

/// Utf8Stream
///
/// ## Example
///
/// ```
/// use utf8_stream::Utf8Stream;
/// let stream = Utf8Stream::new("fire👩🏽‍🚒fighter");
/// assert_eq!(stream.as_str(), "fire👩🏽‍🚒fighter");
/// /// ```
#[doc(alias = "Stream")]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Utf8Stream<'g> {
    pub(crate) ptr: *mut u8,
    pub(crate) index: usize,
    pub(crate) length: usize,
    _marker: PhantomData<&'g u8>,
}

impl<'g> Default for Utf8Stream<'g> {
    fn default() -> Utf8Stream<'g> {
        Utf8Stream {
            ptr: to_slice_ptr_from_display(""),
            index: 0,
            length: 0,
            _marker: PhantomData,
        }
    }
}
impl<'g> Drop for Utf8Stream<'g> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            let layout = Layout::array::<u8>(self.length).unwrap();
            unsafe {
                std::alloc::dealloc(self.ptr, layout);
            }
        }
    }
}
impl<'g> Utf8Stream<'g> {
    /// Creates a new [Utf8Stream](Self) from any implementor of [`Display`](std::fmt::Display)
    ///
    /// ```
    /// use utf8_stream::Utf8Stream;
    /// let stream = Utf8Stream::new("red❤️heart");
    /// assert_eq!(stream.as_str(), "red❤️heart");
    /// ```
    pub fn new<T: Display>(input: T) -> Utf8Stream<'g> {
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
        grow_ptr(self.ptr, old_length, new_length);
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
        shrink_ptr(self.ptr, self.length, 1);
        self.length = 0;
        self.index = 0;
    }

    pub fn rewind(&mut self) {
        self.index = 0;
    }

    pub fn len(&self) -> usize {
        self.length
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
    pub fn get(&self, index: usize) -> Option<&'g str> {
        let (slice, _, _, count) = get_utf8_at_index(self, index);
        if count == 0 || count == 1 && &slice[0..1] == "\0" {
            None
        } else {
            Some(slice)
        }
    }
    pub fn last(&self) -> Option<&'g str> {
        if self.length == 0 {
            None
        } else {
            self.get(self.length - 1)
        }
    }
    pub fn pop(&mut self) -> Option<&'g str> {
        let old_length = self.length;
        if self.length == 0 {
            return None;
        }
        let (slice, index, offset, count) = get_utf8_at_index(self, self.length - 1);
        if count > 0 {
            self.length -= count;
            Some(slice)
        } else {
            None
        }
    }
}
impl<'g> Iterator for Utf8Stream<'g> {
    type Item = &'g str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.length {
            None
        } else {
            let (slice, index, offset, count) = get_utf8_at_index(self, self.index);
            if count > 0 {
                if (self.index + count) <= self.length {
                    self.index += count;
                } else {
                    self.index = self.length -1;
                }
                Some(slice)
            } else {
                None
            }
        }
    }
}

impl<'g> Extend<char> for Utf8Stream<'g> {
    fn extend<T: IntoIterator<Item = char>>(&mut self, iter: T) {
        for string in iter {
            self.push(string);
        }
    }
}
impl<'g> Extend<String> for Utf8Stream<'g> {
    fn extend<T: IntoIterator<Item = String>>(&mut self, iter: T) {
        for string in iter {
            self.push(string);
        }
    }
}
impl<'g> Extend<&'g str> for Utf8Stream<'g> {
    fn extend<T: IntoIterator<Item = &'g str>>(&mut self, iter: T) {
        for string in iter {
            self.push(string);
        }
    }
}
impl<'g> Extend<u8> for Utf8Stream<'g> {
    fn extend<T: IntoIterator<Item = u8>>(&mut self, iter: T) {
        for u in iter {
            self.push(char::from(u));
        }
    }
}

impl<'g> FromIterator<String> for Utf8Stream<'g> {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Utf8Stream<'g> {
        let mut buf = Utf8Stream::default();
        buf.extend(iter);
        buf
    }
}
impl<'g> FromIterator<u8> for Utf8Stream<'g> {
    fn from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Utf8Stream<'g> {
        let mut buf = Utf8Stream::default();
        buf.extend(iter);
        buf
    }
}

impl<'g> FromIterator<char> for Utf8Stream<'g> {
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Utf8Stream<'g> {
        let mut buf = Utf8Stream::default();
        buf.extend(iter);
        buf
    }
}

impl<'g> FromIterator<&'g str> for Utf8Stream<'g> {
    fn from_iter<I: IntoIterator<Item = &'g str>>(iter: I) -> Utf8Stream<'g> {
        let mut buf = Utf8Stream::default();
        buf.extend(iter);
        buf
    }
}

impl<'g> From<&str> for Utf8Stream<'g> {
    fn from(s: &str) -> Utf8Stream<'g> {
        Utf8Stream::new(s)
    }
}

impl<'g> From<String> for Utf8Stream<'g> {
    fn from(s: String) -> Utf8Stream<'g> {
        Utf8Stream::new(s)
    }
}

impl<'g> From<&String> for Utf8Stream<'g> {
    fn from(s: &String) -> Utf8Stream<'g> {
        Utf8Stream::new(s)
    }
}

impl<'g> Display for Utf8Stream<'g> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'g> Deref for Utf8Stream<'g> {
    type Target = str;

    fn deref(&self) -> &str {
        self.as_str()
    }
}
