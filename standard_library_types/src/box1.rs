// box1.rs

// At compile time, Rust needs to know how much space a type takes up. This becomes problematic
// for recursive types, where a value can have as part of itself another value of the same type.
// To get around the issue, we can use a `Box` - a smart pointer used to store data on the heap,
// which also allows us to wrap a recursive type.

pub fn run_box1() {
    let empty_items = create_empty_list();
    let items = create_non_empty_list();

    println!("{:?}", empty_items);
    println!("{:#?}", items);
}

#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub fn create_empty_list() -> List {
    List::Nil
}

pub fn create_non_empty_list() -> List {
    use List::*;
    Cons(100, Box::new(Cons(200, Box::new(Nil))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn check_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}
