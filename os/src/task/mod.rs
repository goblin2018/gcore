use crate::{config::MAX_APP_NUM, sync::UPSafeCell};

use task::TaskControlBlock;
use lazy_static::*;

mod context;
mod switch;

#[allow(clippy::module_inception)]
mod task;

pub struct TaskManager {
  num_app: usize,
  inner: UPSafeCell<TaskManagerInner>,
}

pub struct TaskManagerInner {
  tasks: [TaskControlBlock; MAX_APP_NUM],
  current_task: usize,
}
lazy_static! {
  pub static ref TASK_MANAGER: TaskManager = {
    let num_app = get
  };
}
