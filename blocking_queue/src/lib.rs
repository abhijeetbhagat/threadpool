use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};

pub struct BlockingQueue<T> {
    queue: Mutex<VecDeque<T>>,
    condvar: Condvar,
}

impl<T> BlockingQueue<T> {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            condvar: Condvar::new(),
        }
    }

    pub fn enq(&self, item: T) {
        let mut vec = self.queue.lock().unwrap();
        vec.push_back(item);
        self.condvar.notify_one();
    }

    pub fn deq(&self) -> Option<T> {
        let mut vec = self.queue.lock().unwrap();
        while vec.len() < 1 {
            vec = self.condvar.wait(vec).unwrap();
        }
        vec.pop_front()
    }
}
