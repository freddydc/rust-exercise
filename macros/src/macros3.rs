// macros3.rs

macro_rules! log_message {
    () => {
        println!("Hello!");
    };
    ($val:expr) => {
        println!("{:#?}", $val);
    };
}

pub fn run_macros3() {
    log_message!(("Avocado", "Banana", "Pear"));
    log_message!();
}
