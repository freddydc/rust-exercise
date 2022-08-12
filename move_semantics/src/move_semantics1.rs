// move_semantics1.rs

pub fn run_move_semantics1() {
    let vec0 = Vec::new();

    // `vec0` moved into `fill_vec`
    let mut vec1 = fill_vec(vec0);

    // At this point `vec0` no longer valid.
    // Try accessing `vec0`. See what happens!

    println!("vec1 has length {} content {:?}", vec1.len(), vec1);

    vec1.push(88);

    println!("vec1 has length {} content {:?}", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
