use std::thread;
use std::time::Duration;
use threadpool::ThreadPool;

fn main() {
    let tp = ThreadPool::new();
    tp.execute(|| {
        println!("{:?}: task 1: Hello!", std::thread::current().id());
    });
    tp.execute(|| {
        println!("{:?}: task 2: Hello!", std::thread::current().id());
    });

    thread::sleep(Duration::from_secs(2));

    println!("shutting down pool");
    tp.shutdown();
}
