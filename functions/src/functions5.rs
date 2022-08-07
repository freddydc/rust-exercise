// functions5.rs

pub fn run_functions5() {
    let answer = square(4);
    println!("The square of 4 is {}", answer);
}

fn square(num: i32) -> i32 {
    return num * num;
}
