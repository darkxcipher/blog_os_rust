use core::{future::Future, pin::Pin};
use alloc::boxed::Box;
use core::task::{Context, Poll};
use core::sync::atomic::{AtomicU64, Ordering};
pub mod executor; 



pub struct Task {
    id: TaskId,
    future: Pin<Box<dyn Future<Output = ()>>>,
}

impl Task {
    pub fn new(future: impl Future<Output = ()> +'static ) -> Task {
        Task{
            id: TaskId::new(),
            future: Box::pin(future),
        }
        // add code here
    }
}

impl Task {
    fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }
    // add code here
}

impl TaskId {
    fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        TaskId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
    // add code here
}





pub mod simple_executor;
pub mod keyboard;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord)]
#[derive(Eq)]

struct TaskId(u64);
