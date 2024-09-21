use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;

const WORKERS: i32 = 10;

/// Counter - ... —
struct Counter {
    calls: Mutex<i32>,
}

impl Counter {
    fn new() -> Self {
        Counter {
            calls: Mutex::from(0),
        }
    }

    fn increment(&self) {
        *(self.calls.lock().unwrap()) += 1;
    }

    fn get(&self) -> MutexGuard<'_, i32> {
        self.calls.lock().unwrap()
    }
}

fn main() {
    let cnt = Arc::new(Counter::new());
    let mut thread_handles = Vec::new();

    for _ in 0..WORKERS {
        let cnt_clone = Arc::clone(&cnt);

        let handle = thread::spawn(move || {
            cnt_clone.increment();
        });

        thread_handles.push(handle);
    }

    for thread in thread_handles {
        thread.join().expect("TODO: panic message");
    }

    println!("Counter is equal: {:?}", cnt.get());
}
