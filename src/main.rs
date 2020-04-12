#![no_std]
#![no_main]

extern crate cortex_m_rt;
extern crate panic_halt;

#[no_mangle]
pub extern "C" fn main() -> ! {
    loop {}
}
