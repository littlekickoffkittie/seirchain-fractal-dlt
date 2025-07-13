use std::time::{Instant, Duration};
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let num_tx = 1000;
    let tx_rate = 100; // transactions per second
    let interval = Duration::from_secs_f64(1.0 / tx_rate as f64);

    let start = Instant::now();
    let mut stream = TcpStream::connect("127.0.0.1:8000").await?;

    for i in 0..num_tx {
        let tx = format!("tx {}", i);
        stream.write_all(tx.as_bytes()).await?;
        tokio::time::sleep(interval).await;
    }

    let duration = start.elapsed();
    println!("Sent {} transactions in {:?}", num_tx, duration);
    println!("TPS: {}", num_tx as f64 / duration.as_secs_f64());

    Ok(())
}
