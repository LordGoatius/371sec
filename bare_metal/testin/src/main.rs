#![feature(str_from_raw_parts)]

use std::str;

fn main() {
    let val = "hello world";
    let mut val: &mut str = unsafe { std::str::from_raw_parts_mut(val.as_ptr().cast_mut(), val.len()) };

    unsafe {
        val.as_bytes_mut()[1] = b'1';
    }

    println!("{val}");
}
