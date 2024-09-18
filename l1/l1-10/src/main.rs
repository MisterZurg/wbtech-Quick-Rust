use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // array numbers
    let arr: [i32; 10] = (1..=10)
        .collect::<Vec<_>>()
        .try_into()
        .expect("wrong size iterator");

    println!("{:?}", arr);

    // Make two chan's for numbers -> squares | squares -> stdout
    let (tx, rx) = mpsc::channel();
    let (tx_squares, rx_squares) = mpsc::channel();

    thread::spawn(move || {
        for recieved in rx {
            tx_squares.send(recieved * recieved).unwrap();
        }
    });

    thread::spawn(move || {
        for recieved in rx_squares {
            println!("{}", recieved);
        }
    });

    for v in arr {
        tx.send(v).unwrap();
        thread::sleep(Duration::from_millis(500));
    }
}
