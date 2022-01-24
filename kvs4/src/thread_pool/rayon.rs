use super::ThreadPool;
use crate::Result;

pub struct RayonThreadPool {}

impl ThreadPool for RayonThreadPool {
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
