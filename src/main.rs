// main.rs
extern crate libc;

use libc::c_void;

// Declare the C++ function
extern "C" {
    fn callCFunction();
}

#[no_mangle]
pub extern "C" fn callRustFunction() {
    unsafe {
        callCFunction();
    }
}

fn main() {
    unsafe {
        callRustFunction();
    }
}
