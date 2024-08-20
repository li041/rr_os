#![no_std]
#![no_main]

#[macro_use]
mod console;
mod sbi;
mod lang_items;

use core::arch::global_asm;


global_asm!(include_str!("entry.S"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("hello world");
    panic!("shutdown machine");
}

