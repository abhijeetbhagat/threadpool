use blocking_queue::BlockingQueue;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::JoinHandle;

pub struct ThreadPool {
    bq: Arc<BlockingQueue<Box<dyn FnOnce() + Send + 'static>>>,
    handles: Vec<JoinHandle<()>>,
    should_quit: Arc<AtomicBool>,
}

impl ThreadPool {
    pub fn new() -> Self {
        let bq: Arc<BlockingQueue<Box<dyn FnOnce() + Send + 'static>>> =
            Arc::new(BlockingQueue::new());
        let should_quit = Arc::new(AtomicBool::new(false));

        let handles = (1..=5)
            .map(|_| {
                let bq = Arc::clone(&bq);
                let should_quit = Arc::clone(&should_quit);
                std::thread::spawn(move || {
                    println!("{:?}: hello!", std::thread::current().id());

                    while !should_quit.load(Ordering::Relaxed) {
                        println!("{:?}: waiting ...", std::thread::current().id());
                        let task = bq.deq().unwrap();
                        task();
                    }
                })
            })
            .collect();

        Self {
            bq,
            handles,
            should_quit,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: Send + 'static + FnOnce() -> (),
    {
        self.bq.enq(Box::new(f));
    }

    pub fn shutdown(mut self) {
        self.should_quit.store(true, Ordering::Relaxed);
        let _: Vec<()> = self
            .handles
            .drain(..)
            .map(|handle| {
                let _ = handle.join();
            })
            .collect();
    }
}
