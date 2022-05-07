use core::fmt::{self, Debug, Formatter};

use alloc::vec::Vec;

use super::address::PhysPageNum;

pub struct FramTracker {
  pub ppn: PhysPageNum,
}

impl FramTracker {
  pub fn new(ppn: PhysPageNum) -> Self {
    let bytes_array = ppn.get_bytes_array();
    for i in bytes_array {
      *i = 0;
    }
    Self { ppn }
  }
}

impl Debug for FramTracker {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.write_fmt(format_args!("FrameTracker:PPN={:#x}", self.ppn.0))
  }
}

impl Drop for FramTracker {
  fn drop(&mut self) {
    frame_dealloc(self.ppn)
  }
}

trait FrameAllocator {
  fn new() -> Self;
  fn alloc(&mut self) -> Option<PhysPageNum>;
  fn dealloc(&mut self, ppn: PhysPageNum);
}

pub struct StackFrameAllocator {
  current: usize,
  end: usize,
  recycled: Vec<usize>,
}
