// threads3.rs

use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub fn run_threads3() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let total_queue = queue.total;
    let mut total_received: u32 = 0;

    send_tx(queue, tx);

    for received in rx {
        println!("Got: ({})", received);
        total_received += 1;
    }

    println!("Total numbers received: ({})", total_received);
    assert_eq!(total_received, total_queue);
}

struct Queue {
    total: u32,
    avocado: Vec<u32>,
    banana: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            total: 10,
            avocado: vec![1, 2, 3, 4, 5],
            banana: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(value: Queue, tx: mpsc::Sender<u32>) {
    let queue = Arc::new(value);
    let q1 = Arc::clone(&queue);
    let q2 = Arc::clone(&queue);
    let tx1 = tx.clone();

    thread::spawn(move || {
        for val in &q1.avocado {
            println!("Sending: ({})", val);

            tx1.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        for val in &q2.banana {
            println!("Sending: ({})", val);

            tx.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}
