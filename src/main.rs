use rand::Rng;
use std::{thread, time};
use tokio::sync::mpsc;

struct Message<T> {
    task_id: u64,
    message: T,
}

fn generate_random_amount_milliseconds(low: u64, high: u64) -> time::Duration {
    let x = rand::thread_rng().gen_range(low, high);
    time::Duration::from_millis(x)
}

#[tokio::main]
async fn main() {
    let now = time::Instant::now();
    const NUM_TASKS: u64 = 100;
    const MAX_WAITING_TIME_MS: u64 = 500;
    const BUFFER_SIZE: usize = 50;

    let (tx, mut rx) = mpsc::channel(BUFFER_SIZE);

    for thread_num in 1..NUM_TASKS {
        let tx2 = tx.clone();
        tokio::spawn(async move {
            let amount = generate_random_amount_milliseconds(
                1,
                std::cmp::min(thread_num * 10, MAX_WAITING_TIME_MS),
            );
            thread::sleep(amount);
            tx2.send(Message {
                task_id: thread_num,
                message: amount,
            })
            .await;
        });
    }

    tokio::spawn(async move {
        let amount = time::Duration::from_millis(100);
        thread::sleep(amount);
        tx.send(Message {
            task_id: 0,
            message: amount,
        })
        .await
    });

    let mut sum_amount = 0;
    while let Some(message) = rx.recv().await {
        println!(
            "GOT = {} from Task {}",
            message.message.as_millis(),
            message.task_id
        );
        sum_amount += message.message.as_millis();
    }
    println!("Finished");
    println!("Sum of all sleeps in milliseconds: {}", sum_amount);
    println!(
        "Real elapsed time in milliseconds: {}",
        now.elapsed().as_millis()
    );
}
