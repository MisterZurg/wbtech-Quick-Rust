use std::{
    collections::HashMap,
    sync::{Arc, Mutex, RwLock},
    thread,
    time::Duration,
};

use dashmap::DashMap;


fn hashmap_concurrent() {
    // RwLock in original thread:
    // https://stackoverflow.com/questions/50282619/is-it-possible-to-share-a-hashmap-between-threads-without-locking-the-entire-has
    let data = Arc::new(RwLock::new(HashMap::new()));

    let threads: Vec<_> = (0..10)
        .map(|i| {
            let data = Arc::clone(&data);
            thread::spawn(move || worker_thread(i, data))
        })
        .collect();

    for t in threads {
        t.join().expect("Thread panicked");
    }

    println!("{:?}", data);
}

fn dashmap_concurrent() {
    // Create a DashMap instance
    let dm: DashMap<String, String> = DashMap::new();

    // Define a function to insert key-value pairs concurrently
    fn insert_concurrently(dm: &DashMap<String, String>, key: String, value: String) {
        dm.insert(key.clone(), value.clone());
        println!("Inserted {}: {}", key.clone(), value.clone());
    }

    // Create and start 5 threads to insert key-value pairs concurrently
    let mut handles = vec![];
    for i in 0..5 {
        let dm_clone = dm.clone();
        let key = format!("key_{}", i);
        let value = format!("value_{}", i);
        let handle = thread::spawn(move || {
            insert_concurrently(&dm_clone, key, value);
        });
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Print the contents of the DashMap
    for entry in dm.iter() {
        println!("{:?}: {:?}", entry.key(), entry.value());
    }
}

fn main() {
    println!("HashMap");
    hashmap_concurrent();
    println!("\nDashMap");
    dashmap_concurrent();
}


fn worker_thread(id: u8, data: Arc<RwLock<HashMap<u8, Mutex<i32>>>>) {
    loop {
        // Assume that the element already exists
        let map = data.read().expect("RwLock poisoned");

        if let Some(element) = map.get(&id) {
            let mut element = element.lock().expect("Mutex poisoned");

            // Perform our normal work updating a specific element.
            // The entire HashMap only has a read lock, which
            // means that other threads can access it.
            *element += 1;
            thread::sleep(Duration::from_secs(1));

            return;
        }

        // If we got this far, the element doesn't exist

        // Get rid of our read lock and switch to a write lock
        // You want to minimize the time we hold the writer lock
        drop(map);
        let mut map = data.write().expect("RwLock poisoned");

        // We use HashMap::entry to handle the case where another thread
        // inserted the same key while where were unlocked.
        thread::sleep(Duration::from_millis(50));
        map.entry(id).or_insert_with(|| Mutex::new(0));
        // Let the loop start us over to try again
    }
}