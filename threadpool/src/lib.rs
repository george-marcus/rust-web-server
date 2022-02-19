use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

mod worker;
use crate::worker::Worker;

mod message;
use crate::message::Message;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {

    //Using usize to prevent negative numbers as they make no sense    
    pub fn new(pool_size: usize) -> ThreadPool{

        // unrecoverable error
        assert!(pool_size > 0);

        let (sender, receiver) = mpsc::channel();


        //Using with capacity to preallocate space in vector
        // It's more efficient to preallocate space upfront instead of using Vec::new() 
        // which resizes itself as elements are inserted
        let mut workers = Vec::with_capacity(pool_size);

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..pool_size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool{
            workers,
            sender
        }
    }

    //Using FnOnce as we need the execute method to take ownership of the stream variable only once
    //Using send trait to make the connection handling thread safe
    //Using static annotation because we don’t know how long the thread will take to execute.  
    pub fn execute<T>(&self, job: T) where T: FnOnce() + Send + 'static, {
        let job = Box::new(job);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

/*
    Gracefully cleaning up thread pool
*/
impl Drop for ThreadPool {
    fn drop(&mut self) {

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            // Takes ownership of Some(thread) and leaves None instead
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}