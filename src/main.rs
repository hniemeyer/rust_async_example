use rand::Rng;
use std::{thread, time};
use tokio::sync::mpsc;

struct Message<T> {
    task_id: u64,
    message: T
}

#[tokio::main]
async fn main() {
    let (mut tx, mut rx) = mpsc::channel(1);
    let mut tx2 = tx.clone();

    tokio::spawn(async move {

        for idx in 0..10 {
            let x = rand::thread_rng().gen_range(1, 100);
            let amount = time::Duration::from_millis(x);
            thread::sleep(amount);
            tx.send(Message  {task_id: 1, message: x}).await;
        }
    });

    tokio::spawn(async move {
        tx2.send(Message  {task_id: 2, message: 111}).await;
    });

    while let Some(message) = rx.recv().await {
        println!("GOT = {} from Task {}", message.message, message.task_id);
    }
}
