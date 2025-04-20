use std::error::Error;
use std::{thread, time::Duration};
use chrono::{TimeZone, Utc};
use chrono_tz::Europe::Moscow;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;


const ADDR: &str = "localhost:3310"; // Your own address

async fn say_world() -> String {
    let utc: chrono::DateTime<Utc> = Utc::now();
    let msk_time = utc.with_timezone(&Moscow);

    let ten_millis = time::Duration::seconds(2);
    tokio::time::sleep(Duration::try_from(ten_millis).unwrap()).await;

    format!("Curr time: {:?}\n", msk_time)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    // Next up we create a TCP listener which will listen for incoming
    // connections. This TCP listener is bound to the address we determined
    // above and must be associated with an event loop.
    let listener = TcpListener::bind(&ADDR).await?;
    println!("Listening on: {}", ADDR);

    loop {
        // Asynchronously wait for an inbound socket.
        let (mut socket, _) = listener.accept().await?;

        // And this is where much of the magic of this server happens. We
        // crucially want all clients to make progress concurrently, rather than
        // blocking one on completion of another. To achieve this we use the
        // `tokio::spawn` function to execute the work in the background.
        //
        // Essentially here we're executing a new task to run concurrently,
        // which will allow all of our clients to be processed concurrently.

        tokio::spawn(async move {
            // let mut buf = vec![0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                // let n = socket
                //     .read(&mut buf)
                //     .await
                //     .expect("failed to read data from socket");

                // if n == 0 {
                //     return;
                // }
                // let sw = say_world().await.to_owned();

                socket
                    .write_all(say_world().await.as_bytes())
                    .await
                    .expect("failed to write data to socket");
            }
        });
    }
}