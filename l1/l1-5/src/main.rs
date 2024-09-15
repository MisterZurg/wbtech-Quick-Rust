use tokio::signal;

#[tokio::main]
async fn main() {
    // ... spawn application as separate task ...

    match signal::ctrl_c().await {
        Ok(()) => {
            println!("Gracefully shutting down the server 🍷🗿");
        }
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
            // we also shut down in case of error
        }
    }

    // send shutdown signal to application and wait
}
