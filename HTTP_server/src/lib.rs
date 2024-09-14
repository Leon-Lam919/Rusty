use std::thread;
use actix_web::rt::task::JoinHandle;

// makes the ThreadPool struct public
pub struct ThreadPool{
    workers: Vec<Worker>,
}

// ThreadPool struct implementation
impl ThreadPool{
    
    //func: create new instance of ThreadPool
    pub fn new(size: usize) -> ThreadPool{
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        for id in 0..size{
            // create threads and store in vector
            workers.push(Worker::new(id));
        }
        ThreadPool{workers}
    }
    
    // func: execute function
    // @ 
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
        {}
    
}

struct Worker{
    id: usize, 
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker{
        let thread = thread::spawn(||{});

        Worker{id, thread}
    }
}