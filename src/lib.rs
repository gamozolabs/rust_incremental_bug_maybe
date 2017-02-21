#![feature(lang_items)]
#![feature(unwind_attributes)]
#![no_std]

pub mod fleeb;

#[no_mangle]
pub extern fn _start() -> usize {
    panic!("ASDF");
}

