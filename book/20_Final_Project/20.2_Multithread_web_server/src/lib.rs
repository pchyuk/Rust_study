pub struct ThreadPool;

impl ThreadPool {
    pub fn new(size: u32) -> ThreadPool {
        ThreadPool
    }

    // pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    //     where
    //         F: FnOnce() -> T,
    //         F: Send + 'static,
    //         T: Send + 'static,

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {
    }
}