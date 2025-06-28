use std::alloc::Layout;
use std::fmt::Display;
use std::iter::Iterator;

pub(crate) fn new_ptr(size: usize) -> *mut u8 {
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
pub(crate) fn grow_ptr(ptr: *mut u8, old_size: usize, new_size: usize) -> *mut u8 {
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

pub(crate) fn shrink_ptr(ptr: *mut u8, old_size: usize, new_size: usize) -> *mut u8 {
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

pub(crate) fn is_not_ascii_byte(byte: u8) -> bool {
    !is_ascii_printable_byte(byte) || byte > 127
}

pub(crate) fn is_ascii_printable_byte(byte: u8) -> bool {
    match byte {
        9..13 | 32..126 => true,
        _ => false,
    }
}
pub(crate) fn to_slice_ptr_from_display<T: Display>(input: T) -> *mut u8 {
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

pub(crate) fn get_byte_at_index<'g>(ptr: *mut u8, index: usize) -> u8 {
    let byte = unsafe { ptr.add(index).read() };
    byte
}

pub(crate) fn get_byte_slice_of<'g>(ptr: *mut u8, index: usize, count: usize) -> &'g [u8] {
    let bytes = unsafe { std::slice::from_raw_parts(ptr.add(index), count) };
    bytes
}

pub(crate) fn get_str_slice_of<'g>(ptr: *mut u8, index: usize, count: usize) -> &'g str {
    if let Ok(string) = std::str::from_utf8(get_byte_slice_of(ptr, index, count)) {
        string
    } else {
        ""
    }
}
