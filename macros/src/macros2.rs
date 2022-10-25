// macros2.rs

#[macro_use]
mod macros {
    macro_rules! log_message {
        () => {
            println!("Greeting!");
        };
    }
}

pub fn run_macros2() {
    log_message!();
}
