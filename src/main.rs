use rand::Rng;
use std::{thread, time};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (mut tx, mut rx) = mpsc::channel(1);
    let mut tx2 = tx.clone();

    tokio::spawn(async move {

        for idx in 0..10 {
            let x = rand::thread_rng().gen_range(1, 100);
            let amount = time::Duration::from_millis(x);
            thread::sleep(amount);
            tx.send(x).await;
        }
    });

    tokio::spawn(async move {
        tx2.send(111).await;
    });

    while let Some(message) = rx.recv().await {
        println!("GOT = {}", message);
    }
}
