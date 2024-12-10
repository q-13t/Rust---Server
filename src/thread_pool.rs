use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

use crate::{server::get_log_level, Logger};
/// # ThreadPool
///
/// The WorkerPool is a struct that represents a pool of worker threads. This
/// pool is responsible for executing the jobs (functions) in a separate thread
/// context. The pool is created with a specified number of threads which are
/// started and waiting for jobs to execute. When a job is added to the pool,
/// the next available worker thread will execute it. The pool also handles the
/// shutdown of the worker threads when the pool is dropped.
///
/// # Arguments
/// * `size` - The number of threads in the worker pool
/// # Example
/// ``` rust
/// use thread_pool::ThreadPool;
///
/// let pool = ThreadPool::new(4);
/// pool.execute(|| {
///     // do something
/// });
/// ```
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers: Vec<Worker> = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        let logger = Logger::new("WORKER", get_log_level());
        for worker in &mut self.workers {
            logger.info(&["Worker", &worker.id.to_string(), "started"]);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

/// # Worker
/// A worker is a thread that is responsible for executing a single job it is a part of the worker pool.
///
/// # Arguments
/// * `id` - The id of the worker
/// * `receiver` - The receiver of the worker
///
/// # Example
/// ``` rust
/// use thread_pool::Worker;
///
/// let worker = Worker::new(0, Arc::new(Mutex::new(receiver)));
/// ```
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
type Job = Box<dyn FnOnce() + Send + 'static>;

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let logger = Logger::new("POOL", get_log_level());
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    logger.info(&["Worker", &id.to_string(), "started"]);
                    job();
                }
                Err(err) => {
                    logger.error(
                        err.to_string().as_str(),
                        &["Worker", &id.to_string(), "stopped"],
                    );
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
