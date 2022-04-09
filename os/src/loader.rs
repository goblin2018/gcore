use core::{mem, slice, arch::asm};

use crate::{
  config::{APP_BASE_ADDRESS, APP_SIZE_LIMIT, KERNEL_STACK_SIZE, MAX_APP_NUM, USER_STACK_SIZE},
  trap::TrapContext,
};

#[repr(align(4096))]
#[derive(Copy, Clone)]
struct KernelStack {
  data: [u8; KERNEL_STACK_SIZE],
}

#[repr(align(4096))]
#[derive(Copy, Clone)]
struct UserStack {
  data: [u8; USER_STACK_SIZE],
}

static KERNEL_STACK: [KernelStack; MAX_APP_NUM] = [KernelStack {
  data: [0; KERNEL_STACK_SIZE],
}; MAX_APP_NUM];

static USER_STACK: [UserStack; MAX_APP_NUM] = [UserStack {
  data: [0; USER_STACK_SIZE],
}; MAX_APP_NUM];

impl KernelStack {
  fn get_sp(&self) -> usize {
    self.data.as_ptr() as usize + KERNEL_STACK_SIZE
  }
  pub fn push_context(&self, trap_cx: TrapContext) -> usize {
    let trap_cx_ptr = (self.get_sp() - mem::size_of::<TrapContext>()) as *mut TrapContext;
    unsafe {
      *trap_cx_ptr = trap_cx;
    }

    trap_cx_ptr as usize
  }
}

impl UserStack {
  fn get_sp(&self) -> usize {
    self.data.as_ptr() as usize + USER_STACK_SIZE
  }
}

fn get_base_i(app_id: usize) -> usize {
  APP_BASE_ADDRESS + app_id * APP_SIZE_LIMIT
}

pub fn get_num_app() -> usize {
  extern "C" {
    fn _num_app();
  }

  unsafe { (_num_app as usize as *const usize).read_volatile() }
}

pub fn load_apps() {
  extern "C" {
    fn _num_app();
  }

  let num_app_str = _num_app as usize as *const usize;
  let num_app = get_num_app();
  let app_start = unsafe { slice::from_raw_parts(num_app_str.add(1), num_app + 1) };

  unsafe {
    asm!("fence.i")
  }

  for i in 0..num_app {
    let base_i = get_base_i(i);
    
  }

}
