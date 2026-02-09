#![no_std]

mod myalloc;
mod js;
mod polyfill;

pub use myalloc::alloc;

#[no_mangle]
pub unsafe extern "C" fn run() -> usize {
    js::setup();
    0
}
