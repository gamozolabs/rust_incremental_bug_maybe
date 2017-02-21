#[lang = "panic_fmt"]
#[unwind]
#[no_mangle]
pub extern fn panic_fmt(
    _args: ::core::fmt::Arguments, _file: &'static str, _line: u32) -> !
{
    loop {}
}

