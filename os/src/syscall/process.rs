use crate::batch::run_next_app;


pub fn sys_exit(code: i32) -> ! {
  println!("[kernel] Applicaton exited with code {}", code);
  run_next_app()
}

