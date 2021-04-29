use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};

pub struct BlockingQueue<T> {
    queue: Mutex<VecDeque<T>>,
    condvar: Condvar,
    should_quit: Arc<AtomicBool>,
}

impl<T> BlockingQueue<T> {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            condvar: Condvar::new(),
            should_quit: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn enq(&self, item: T) {
        let mut vec = self.queue.lock().unwrap();
        vec.push_back(item);
        self.condvar.notify_one();
    }

    pub fn deq(&self) -> Option<T> {
        let mut vec = self
            .condvar
            .wait_while(self.queue.lock().unwrap(), |vec| {
                println!("{:?}: checking cond ...", std::thread::current().id());
                vec.is_empty() && !self.should_quit.load(Ordering::Relaxed)
            })
            .unwrap();

        if self.should_quit.load(Ordering::Relaxed) {
            println!("bq is now shutting down");
            return None;
        }

        vec.pop_front()
    }

    pub fn quit(&self) {
        self.should_quit.store(true, Ordering::Relaxed);
        self.condvar.notify_all();
    }
}
