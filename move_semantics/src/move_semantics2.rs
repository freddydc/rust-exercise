// move_semantics2.rs

pub fn run_move_semantics2() {
    let animal = Vec::new();

    // `fill_vec` borrows a reference to `animal`
    let mut animals = fill_vec(&animal);

    println!("Animal has length {} content {:?}", animal.len(), animal);

    animals.push('🦊');

    println!("Animals has length {} content {:?}", animals.len(), animals);
}

fn fill_vec(vec: &[char]) -> Vec<char> {
    let mut vec = vec.to_vec();

    vec.push('🐏');
    vec.push('🐪');

    vec
}
