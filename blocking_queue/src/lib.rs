use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex, RwLock};

pub struct BlockingQueue<T> {
    queue: Mutex<VecDeque<T>>,
    condvar: Condvar,
    should_quit: Arc<RwLock<bool>>,
}

impl<T> BlockingQueue<T> {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            condvar: Condvar::new(),
            should_quit: Arc::new(RwLock::new(false)),
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
                vec.is_empty() && !*self.should_quit.read().unwrap()
            })
            .unwrap();

        if *self.should_quit.read().unwrap() {
            println!("bq is now shutting down");
            return None;
        }

        vec.pop_front()
    }

    pub fn quit(&self) {
        let mut flag = self.should_quit.write().unwrap();
        *flag = true;
        self.condvar.notify_all();
    }
}
