// threads2.rs

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

pub fn run_threads2() {
    let store = Mutex::new(JobStatus { jobs_completed: 0 });
    let status = Arc::new(store);
    let mut handles = vec![];

    for _ in 1..=10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            let mut job_status = status_shared.lock().unwrap();

            job_status.jobs_completed += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();

        let job_status = status.lock().unwrap();
        println!("Jobs completed ({})", job_status.jobs_completed);
    }
}
