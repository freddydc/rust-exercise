// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.

pub fn run_primitive_types5() {
    let cat = ("Furry Mc", 3.5);
    let (name, age) = cat;

    println!("{} is {} years old.", name, age);
}
