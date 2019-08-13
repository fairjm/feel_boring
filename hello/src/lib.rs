use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Sender, Receiver};
use std::sync::{Arc, Mutex};

type Job = Box<dyn FnBox + Send + 'static>;

trait FnBox {
    fn call_box(self : Box<Self>);
}
///
/// impl FnBox for FnOnce() to add method to deal with the Self(function) without moving it out of the box(But moving the box itself)
/// 
impl<F:FnOnce()> FnBox for F {
    fn call_box(self: Box<Self>) {
        (*self)();
    }
}


pub struct ThreadPool {
    workers : Vec<Worker>,
    sender : Sender<Job>,
}

impl ThreadPool {

    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panic
    /// 
    /// The `new` function will panic if the size is zero.
    pub fn new(num: usize) -> ThreadPool {
        assert!(num > 0);
        let mut workers = Vec::with_capacity(num);
        let (sender, receiver) = mpsc::channel();
        let arc = Arc::new(Mutex::new(receiver));
        for i in 0..num {
            workers.push(Worker::new(i, Arc::clone(&arc)));
        }
        ThreadPool {
            workers,
            sender
        }
    }

    pub fn execute<F>(&self, f : F)
     where F: FnOnce() + Send + 'static {
         let job = Box::new(f);
         self.sender.send(job).unwrap();
     }
}

struct Worker {
    id : usize,
    handler : JoinHandle<()>
}

impl Worker {
    fn new(id:usize, receiver : Arc<Mutex<Receiver<Job>>>) -> Worker {
        let handler = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("worker-{} start working", id);
                job.call_box();
            }
        });
        Worker {
            id,
            handler
        }
    }
}