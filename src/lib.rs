#![allow(unused)]
pub mod stream;

#[doc(inline)]
pub use stream::Utf8Stream;

pub mod internal;
pub use internal::{
    get_byte_at_index, get_byte_slice_of, get_str_slice_of, get_utf8_at_index, grow_ptr,
    is_ascii_printable_byte, is_not_ascii_byte, new_ptr, shrink_ptr, to_slice_ptr_from_display,
};
