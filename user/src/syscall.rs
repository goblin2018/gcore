use core::arch::asm;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;

fn syscall(code: usize, args: [usize; 3]) -> isize {
  let mut ret: isize;
  unsafe {
    asm!(
      "ecall",
      inlateout("x10") args[0] =>ret,
      in("x11") args[1],
      in("x12") args[2],
      in("x17") code
    );
  }
  ret
}

pub fn sys_write(code: usize, buffer: &[u8]) -> isize {
  syscall(
    SYSCALL_WRITE,
    [code, buffer.as_ptr() as usize, buffer.len()],
  )
}

pub fn sys_exit(code: i32) -> isize {
  syscall(SYSCALL_EXIT, [code as usize, 0, 0])
}
