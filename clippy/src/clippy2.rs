// clippy2.rs

pub fn run_clippy2() {
    let mut res = 100;
    let option = Some(400);

    if let Some(x) = option {
        res += x;
    }

    println!("Sum equal to ({})", res);
}
