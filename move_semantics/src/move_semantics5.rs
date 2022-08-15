// move_semantics5.rs
// Mutable References

pub fn run_move_semantics5() {
    let mut count = 100;

    let x = &mut count;
    *x += 400;

    let y = &mut count;
    *y += 1000;

    assert_eq!(count, 1500);

    println!("Count value {}", count);
}
