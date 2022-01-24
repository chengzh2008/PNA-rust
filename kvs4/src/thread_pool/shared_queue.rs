use super::ThreadPool;
use crate::Result;

pub struct SharedQueueThreadPool {}

impl ThreadPool for SharedQueueThreadPool {
    fn new(threads: u32) -> Result<Self> {
        todo!()
    }

    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Sized + 'static,
    {
        todo!()
    }
}
