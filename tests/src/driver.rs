use threadpool::ThreadPool;

fn main() {
    let tp = ThreadPool::new();
    tp.execute(|| {
        println!("{:?}: task 1: Hello!", std::thread::current().id());
    });
    tp.execute(|| {
        println!("{:?}: task 2: Hello!", std::thread::current().id());
    });
    tp.shutdown();
}
