// cow1.rs

use std::borrow::Cow;

fn abs_all(mut data: Cow<[i32]>) -> Cow<[i32]> {
    for i in 0..data.len() {
        let value = data[i];
        if value < 0 {
            // Clones into a vector if not already owned.
            data.to_mut()[i] = -value;
        }
    }
    data
}

pub fn run_cow1() {
    // No clone occurs because `input` doesn't need to be mutated.
    let slice = [0, 100, 200];
    let input = Cow::from(&slice[..]);

    match abs_all(input) {
        Cow::Borrowed(v) => println!("I borrowed the slice: {:#?}", v),
        _ => panic!("Expected borrowed value"),
    }

    // Clone occurs because `input` needs to be mutated.
    let slice = [-100, 0, 100];
    let input = Cow::from(&slice[..]);

    match abs_all(input) {
        Cow::Owned(v) => println!("I modified the slice and now own it: {:#?}", v),
        _ => panic!("Expected owned value"),
    }

    // No clone occurs because `input` is already owned.
    let vec = vec![200, 0, -200];
    let input = Cow::from(vec);

    match abs_all(input) {
        Cow::Owned(v) => println!("I own this slice: {:#?}", v),
        _ => panic!("Expected owned value"),
    }
}
