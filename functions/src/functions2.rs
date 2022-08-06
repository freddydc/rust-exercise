// functions2.rs

fn call_me(num: u16) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

pub fn run_functions2() {
    call_me(3);
}
