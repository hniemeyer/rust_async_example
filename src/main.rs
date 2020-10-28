use rand::Rng;
use std::{thread, time};
use tokio::sync::mpsc;

struct Message<T> {
    task_id: u64,
    message: T
}

fn generate_random_amount_milliseconds(low: u64, high: u64) -> time::Duration
{
    let x = rand::thread_rng().gen_range(low, high);
    time::Duration::from_millis(x)
}

#[tokio::main]
async fn main() {
    let (mut tx, mut rx) = mpsc::channel(1);
    let mut tx2 = tx.clone();

    tokio::spawn(async move {

        for idx in 0..10 {
            let amount = generate_random_amount_milliseconds(1, 100);
            thread::sleep(amount);
            tx.send(Message  {task_id: 1, message: amount}).await;
        }
    });

    tokio::spawn(async move {
        let amount = generate_random_amount_milliseconds(1, 20);
        thread::sleep(amount);
        tx2.send(Message  {task_id: 2, message: amount}).await;
    });

    while let Some(message) = rx.recv().await {
        println!("GOT = {} from Task {}", message.message.as_millis(), message.task_id);
    }
}
