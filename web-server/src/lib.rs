use std::thread

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
};

impl ThreadPool {
    /// Create new ThreadPool
    /// The size is the number of threads in pool
    /// # Panics
    /// The new function will panic if size is zero
    pub fn new(size: usize) -> ThreadPool{
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            //create threads and store them
        }
        ThreadPool { threads }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
        {}
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
};

impl Worker {
    fn new (id: usize) -> Worker{
        let thread = thread::spawn(|| {});

        Worker{ id, thread }
    }
}