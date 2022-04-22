#![no_std]
#![no_main]
#![feature(panic_info_message)]


#[cfg(feature = "board_k210")]
#[path ="boards/k210.rs"]
mod board;
#[cfg(not(any(feature= "board_k210")))]
#[path ="boards/qemu.rs"]
mod board;


#[macro_use]
mod console;
mod lang_items;
mod sbi;
mod sync;
mod loader;
pub mod syscall;
mod mm;
pub mod trap;
mod config;
mod timer;
pub mod task;


use core::{arch::global_asm, slice};
global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub fn rust_main() -> ! {
  clear_bss();
  // color_print();
  println!("[Kernel] hello, world");
  trap::init();
  loader::load_apps();
  trap::enable_timer_interrupt();
  timer::set_next_trigger();
  task::run_first_task();
  panic!("shotdown machine")
}

fn clear_bss() {
  extern "C" {
    fn sbss();
    fn ebss();
  }

  unsafe {
    slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize).fill(0);
  }

  // (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) })
}
// fn color_print() {
//   println!("\x1b[31m[ERROR] {}\x1b[0m", "hello world");
//   println!("\x1b[93m[WARN] {}\x1b[0m", "hello world");
//   println!("\x1b[34m[INFO] {}\x1b[0m", "hello world");
//   println!("\x1b[32m[DEBUG] {}\x1b[0m", "hello world");
// }
