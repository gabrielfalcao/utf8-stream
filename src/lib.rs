#![allow(unused)]
mod stream;

#[doc(inline)]
pub use stream::Utf8Stream;

mod internal;
pub(crate) use internal::{
    get_byte_at_index, get_byte_slice_of, get_str_slice_of, is_not_ascii_byte,
};

mod heuristics;
pub use heuristics::get_utf8_at_index;
