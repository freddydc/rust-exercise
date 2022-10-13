// arc1.rs

use std::sync::Arc;
use std::thread;

pub fn run_arc1() {
    let numbers = (1..=100u32).collect::<Vec<_>>();
    let shared_numbers = Arc::new(numbers);
    let mut handles = vec![];

    for offset in 0..=7 {
        let child_numbers = Arc::clone(&shared_numbers);
        let handle = thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|n| *n % 8 == offset).sum();
            println!("Sum of offset ({}) is {}", offset, sum);
        });
        handles.push(handle);
    }

    for handle in handles.into_iter() {
        handle.join().unwrap();
    }
}
