use std::sync::mpsc::Receiver;
use std::thread::{self, JoinHandle};
use std::sync::{Arc, Mutex, mpsc};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

enum Message{
    NewJob(Job),
    Terminate,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool{
    workers:Vec<Worker>,
    sender:mpsc::Sender<Message>,
}
struct Worker{
    id:usize,
    thread:Option<JoinHandle<()>>
}
impl Worker{
    fn new(id:usize,receiver:Arc<Mutex<Receiver<Message>>>)->Worker{
        let thread = Some(thread::spawn(move || {
            loop {
                // correct usage of receiver
                let message = receiver.lock().unwrap().recv().unwrap();
                // id is `usize`, so can be moved easily.
                
                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} get a job; executing.",id);
                        job();
                    },
                    Message::Terminate => {
                        println!("Worker {} wa told to terminate.",id);
                        break;
                    }
                }
                // test lock usage
                // let rec = receiver.lock().unwrap();
                // let job = rec.recv().unwrap();
                // println!("Worker {} get a job; executing.",id);
                // job();
            }
        }));
        Worker{
            id,
            thread,
        }
    }
}

impl ThreadPool{
    /// Create a new ThreadPool.
    /// 
    /// The size is the number of threads in the pool. 
    /// 
    /// # Example
    /// ```
    /// let pool = ThreadPool::new(3);
    /// ```
    ///
    /// # Panics
    /// 
    /// The `new` function will panic if the size is 0.
    pub fn new(pool_size:usize)->ThreadPool{// usize here 主要是为了方便索引，不用再进行类型转换了
        assert!(pool_size>0);
        
        // init the size, but without contents initializer.
        let mut workers = Vec::with_capacity(pool_size);
        let (sender,receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));
        for id in 0..pool_size {
            workers.push(Worker::new(id,Arc::clone(&receiver)));
        }

        ThreadPool{
            workers,
            sender
        }
    }
    pub fn execute<T>(&self , closure:T)
    where
        T:FnOnce() + Send + 'static
    {
        let job = Box::new(closure);
        let message = Message::NewJob(job);
        self.sender.send(message).unwrap();
    }
}
impl Drop for ThreadPool{
    fn drop(&mut self) {
        println!("Sending message to workers.");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Sending down all workers. Wait the terminate run requests.");

        for worker in &mut self.workers{
            println!("Shutting down worker {}",worker.id);
            //worker.thread.join().unwrap();
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}