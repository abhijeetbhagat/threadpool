use blocking_queue::BlockingQueue;
use std::sync::Arc;
use std::thread;

fn main() {
    let q = Arc::new(BlockingQueue::new());
    let clone = Arc::clone(&q);
    let h = thread::spawn(move || {
        for i in 0..5 {
            clone.enq(i);
        }
    });

    for _ in 0..5 {
        println!("recvd: {}", q.deq().unwrap());
    }

    let _ = h.join();
}
