// threads1.rs

use std::thread;
use std::time::Duration;

pub fn run_threads1() {
    let mut handles = vec![];
    for i in 1..=10 {
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            println!("Thread ({}) is complete", i);
        });
        handles.push(handle);
    }

    let mut completed_threads = 0;
    for handle in handles {
        handle.join().unwrap();
        completed_threads += 1;
    }

    if completed_threads != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }
}
