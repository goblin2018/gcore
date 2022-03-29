#![no_std]
#![no_main]
#![feature(panic_info_message)]
mod console;
mod lang_items;
mod sbi;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
  clear_bss();
  color_print();
  panic!("shotdown machine")
}

fn clear_bss() {
  extern "C" {
    fn sbss();
    fn ebss();
  }

  (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) })
}
fn color_print() {
  println!("x1b[31m{}x1b[0m", "hello world");
}