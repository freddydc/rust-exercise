// primitive_types6.rs
// Use a tuple index to access the second element of `numbers`.

pub fn run_primitive_types6() {
    let animals = ('🐪', '🐏');
    let camel = animals.0;

    println!("The animal is {}", camel);
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);
        let second = numbers.1;

        assert_eq!(2, second, "This is not the 2nd number in the tuple!")
    }
}
