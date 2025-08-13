use std::os::raw::c_char;
use std::slice;

#[unsafe(no_mangle)]
fn _start() {}

#[unsafe(no_mangle)]
pub extern "C" fn rust_filter(
    _tag: *const c_char,
    _tag_len: u32,
    _time_sec: u32,
    _time_nsec: u32,
    record: *const c_char,
    record_len: u32,
) -> *const u8 {
    println!("run wasm_filter");
    let slice_record = unsafe { slice::from_raw_parts(record as *const u8, record_len as usize) };
    let vrecord = unsafe { str::from_utf8_unchecked(slice_record) };
    vrecord.as_ptr()
}
