use crate::config::{KERNEL_STACK_SIZE, USER_STACK_SIZE};

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
