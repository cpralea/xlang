use std::ffi;

use cxltypes;


#[no_mangle]
pub extern "C" fn __xlrt_print_bool(value: cxltypes::c_xl_bool) {
    let value = match value {
        0 => "false",
        1 => "true",
        _ => unreachable!(),
    };
    println!("{}", value);
}


#[no_mangle]
pub extern "C" fn __xlrt_print_int(value: cxltypes::c_xl_int) {
    println!("{}", value);
}


#[no_mangle]
pub extern "C" fn __xlrt_print_str(value: cxltypes::c_xl_str) {
    unsafe {
        let value = ffi::CStr::from_ptr(value).to_string_lossy().into_owned();
        println!("{}", value);
    }
}
