

mod engine;
mod feed;
mod liquidator;
mod scanner;
mod v1;

fn main() {

    // let engine = Arc::new((Mutex::new(engine), Condvar::new(), Condvar::new()));

    // let e1 = Arc::clone(&engine);
    // let e2 = Arc::clone(&engine);
    // let e3 = Arc::clone(&engine);

    // let liquidator_thread = thread::spawn(move || liquidator(e1));
    // let scanner_thread = thread::spawn(move || scanner(e2));
    // let feed_thread = thread::spawn(move || {
    //     tokio::runtime::Runtime::new().unwrap().block_on(feed(e3));
    // });

    // liquidator_thread.join().unwrap();
    // scanner_thread.join().unwrap();
    // feed_thread.join().unwrap();
}
