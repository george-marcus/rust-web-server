use threadpool::ThreadPool;

#[test]
#[should_panic]
fn thread_pool_should_not_start_with_zero_threads() {
    ThreadPool::new(0);
}