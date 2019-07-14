use std::thread;

#[no_mangle]
pub extern fn process() {
    let handles: Vec<_> = (0..10).map(|_| {
        thread::spawn(|| {
            let mut count = 0;
            for _ in 0..10_000_000 {
                count += 1
            }
            count
        })
    }).collect();

    let mut thread_id = 0;

    for h in handles {
        println!("Thread {} is finnished.", thread_id);
        h.join().map_err(|_| "Couldn't join thread.").unwrap();
        thread_id += 1
    }
}
