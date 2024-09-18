use std::{io, sync::mpsc, thread, time::Duration};

/// scan_vector_size helper for init_vec()
fn scan_deadline() -> u64 {
    let mut input_line = String::new();

    io::stdin() // the rough equivalent of `std::cin`
        .read_line(&mut input_line) // actually read the line
        .expect("Failed to read line"); // which can fail, however

    let x: u64 = input_line
        .trim() // ignore whitespace around input
        .parse() // convert to integers
        .expect("Input not an integer"); // which, again, can fail

    x
}

fn main() {
    let program_deadline = scan_deadline();

    // Channels for send and receive
    let (tx, rx) = mpsc::channel();
    // Channels for notification to close the above ones
    let (tx_ctx, rx_ctx) = mpsc::channel();

    // Spawn thread for sending data
    let sender = thread::spawn(move || {
        for i in 0.. {
            // Check, <- ctx.Done()
            if rx_ctx.try_recv().is_ok() {
                break;
            }

            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Spawn thread for reading data
    let receiver = thread::spawn(move || {
        for received in rx {
            println!("Received: {}", received);
        }
    });

    // Wait for provided deadline, before sending notify signal
    // to gracefully terminate threads
    thread::sleep(Duration::from_secs(program_deadline));
    println!("Terminating threads ✋🗿...");
    tx_ctx.send(()).unwrap();

    // Wait for threads termination
    sender.join().unwrap();
    receiver.join().unwrap();
}
