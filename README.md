A simple threadpool implementation.

# Example
```Rust
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

    tp.shutdown();
}
```
Output:
```
ThreadId(2): hello!
ThreadId(2): waiting ...
ThreadId(3): hello!
ThreadId(3): waiting ...
ThreadId(3): task 2: Hello!
ThreadId(3): waiting ...
ThreadId(2): task 1: Hello!
ThreadId(2): waiting ...
ThreadId(5): hello!
ThreadId(6): hello!
ThreadId(5): waiting ...
ThreadId(6): waiting ...
ThreadId(4): hello!
ThreadId(4): waiting ...
```
