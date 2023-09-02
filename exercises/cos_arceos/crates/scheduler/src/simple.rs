use alloc::{collections::VecDeque, sync::Arc};
use core::ops::Deref;

use crate::BaseScheduler;
const TIME_SLICE: usize = 10;
/// A task wrapper for the [`SimpleScheduler`].
pub struct SimpleTask<T> {
    inner: T,
}

impl<T> SimpleTask<T> {
    /// Creates a new [`SimpleTask`] from the inner task struct.
    pub const fn new(inner: T) -> Self {
        Self {
            inner,
        }
    }

    /// Returns a reference to the inner task struct.
    pub const fn inner(&self) -> &T {
        &self.inner
    }
}

impl<T> Deref for SimpleTask<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// A simple scheduler.
///
/// When a task is added to the scheduler, it's placed at the end of the ready
/// queue. When picking the next task to run, the head of the ready queue is
/// taken.
///
/// As it's a cooperative scheduler, it does nothing when the timer tick occurs.
///
/*pub struct SimpleScheduler<T> {
    ready_queue: VecDeque<Arc<SimpleTask<T>>>,
} */
pub struct SimpleScheduler<T> {
    time_slice: usize, // 时间片大小
    ready_queue: VecDeque<Arc<SimpleTask<T>>>,
}


impl<T> SimpleScheduler<T> {
    /// Creates a new empty [`SimpleScheduler`].
    pub const fn new() -> Self {
        Self {
            time_slice: TIME_SLICE, // 初始化时间片计数器 ；add
            ready_queue: VecDeque::new(),
        }
    }
    /// get the name of scheduler
    pub fn scheduler_name() -> &'static str {
        "Simple"
    }
}

impl<T> BaseScheduler for SimpleScheduler<T> {
    type SchedItem = Arc<SimpleTask<T>>;

    fn init(&mut self) {}

    fn add_task(&mut self, task: Self::SchedItem) {
        trace!("######### add_task");
        self.ready_queue.push_back(task);
    }

    fn remove_task(&mut self, task: &Self::SchedItem) -> Option<Self::SchedItem> {
        trace!("######### remove_task");
        self.ready_queue
            .iter()
            .position(|t| Arc::ptr_eq(t, task))
            .and_then(|idx| self.ready_queue.remove(idx))
    }

    /*fn pick_next_task(&mut self) -> Option<Self::SchedItem> {
        self.ready_queue.pop_front()
    }*/
    fn pick_next_task(&mut self) -> Option<Self::SchedItem> {
        if let Some(next_task) = self.ready_queue.pop_front() {
            // 减少时间片计数器
            self.time_slice -= 1;
            
            // 如果时间片用尽或队列为空，将任务重新放入队列
            if self.time_slice == 0 || self.ready_queue.is_empty() {
                self.time_slice = TIME_SLICE; // 重置时间片计数器
                self.ready_queue.push_back(next_task.clone());
            }
    
            Some(next_task)
        } else {
            None
        }
    }
    

    fn put_prev_task(&mut self, prev: Self::SchedItem, _preempt: bool) {
        self.ready_queue.push_back(prev);
    }

    fn task_tick(&mut self, _current: &Self::SchedItem) -> bool {
       // false // no reschedule
        self.ready_queue.push_back(Arc::clone(_current));
        true // 返回true以触发任务切换
    }

    fn set_priority(&mut self, _task: &Self::SchedItem, _prio: isize) -> bool {
        false
    }
}
