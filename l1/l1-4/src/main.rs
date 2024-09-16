use std::sync::{Arc, Mutex}; // Importing necessary libraries for synchronization
use std::sync::mpsc; // Importing the channel for inter-thread communication
use std::thread; // Importing the threading functionality

const NUM_CONSUMERS: usize = 3; // Defining the number of consumer threads
const NUM_MESSAGES: usize = 10; // Defining the number of messages to be produced

fn main() {
    // Create a channel for communication between producer and consumers
    let (sender, receiver) = mpsc::channel();

    // Wrap the receiver in an Arc and a Mutex for safe sharing among multiple threads
    let receiver = Arc::new(Mutex::new(receiver));

    // Spawn multiple consumer threads
    let mut handles = vec![];
    for i in 0..NUM_CONSUMERS {
        let receiver = Arc::clone(&receiver);
        let handle = thread::spawn(move || {
            consume_messages(i, receiver);
        });
        handles.push(handle);
    }

    // Start the producer thread
    thread::spawn(move || {
        produce_messages(sender);
    });

    // Wait for all consumer threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
}

// Function to simulate message production
fn produce_messages(sender: mpsc::Sender<String>) {
    for i in 0..NUM_MESSAGES {
        let message = format!("Message {}", i);
        sender.send(message.clone()).unwrap(); // Clone the message before sending
        println!("Producer sent: {}", message);
        thread::sleep(std::time::Duration::from_millis(500)); // Simulate message production delay
    }
}

// Function to consume messages received from the channel
fn consume_messages(id: usize, receiver: Arc<Mutex<mpsc::Receiver<String>>>) {
    loop {
        let message = receiver.lock().unwrap().recv(); // Receive message from the channel
        match message {
            Ok(msg) => {
                println!("Consumer {} received: {}", id, msg); // Print the received message
                thread::sleep(std::time::Duration::from_secs(1)); // Simulate message processing
            }
            Err(_) => break, // Break the loop if channel is closed
        }
    }
}