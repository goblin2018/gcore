mod context;

use core::arch::global_asm;

pub use context::TrapContext;
use riscv::register::{
  scause::{self, Exception, Trap,Interrupt},
  stval, stvec,
  utvec::TrapMode, sie, 
};

use crate::{timer::set_next_trigger, task::{exit_current_and_run_next, suspend_current_and_run_next}, syscall::syscall};


global_asm!(include_str!("trap.S"));

pub fn init() {
  extern "C" {
    fn __alltraps();
  }

  unsafe {
    stvec::write(__alltraps as usize, TrapMode::Direct);
  }
}


pub fn enable_timer_interrupt() {
  unsafe {
    sie::set_stimer();
  }
}


#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
  let scause = scause::read();
  let stval = stval::read();
  match scause.cause() {
    Trap::Exception(Exception::UserEnvCall) => {
      cx.sepc += 4;
      cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
    }

    Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
      println!("[kernel] PageFault in application, kernel killed it.");
      exit_current_and_run_next();
    }
    Trap::Exception(Exception::IllegalInstruction) => {
      println!("[kernel] IllegalInstruction in applicaton, kernel killed it.");
      exit_current_and_run_next();
    }
    Trap::Interrupt(Interrupt::SupervisorTimer) => {
      set_next_trigger();
      suspend_current_and_run_next();
    }

    _ => {
      panic!(
        "Unsupproted trap {:?}, stval = {:#x}",
        scause.cause(),
        stval
      );
    }
  }

  cx
}
