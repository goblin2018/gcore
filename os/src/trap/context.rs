use riscv::register::sstatus::{self, Sstatus, SPP};

#[repr(C)]
pub struct TrapContext {
  // general res[0:31]
  pub x: [usize; 32],
  // CSR sstatus
  pub sstatus: Sstatus,
  // CSR sepc
  pub sepc: usize,
}

impl TrapContext {
  pub fn set_sp(&mut self, sp: usize) {
    self.x[2] = sp;
  }


  pub fn app_init_context(entry: usize, sp: usize) -> Self {
    let mut sstatus = sstatus::read();
    sstatus.set_spp(SPP::User);
    let mut cx = Self {
      x:[0;32],
      sstatus,
      sepc: entry, // entry point of app
    };
    cx.set_sp(sp); // app user stack pointer
    cx // return init Trap Context of app
  }
}
