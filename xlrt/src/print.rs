use std::ffi;

use cxltypes;


#[no_mangle]
pub extern fn __xlrt_print_int(value: cxltypes::c_xl_int) {
    println!("{}", value);
}


#[no_mangle]
pub extern fn __xlrt_print_str(value: cxltypes::c_xl_str) { unsafe {
    let value = ffi::CStr::from_ptr(value).to_string_lossy().into_owned();
    println!("{}", value);
}}
