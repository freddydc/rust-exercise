// primitive_types4.rs

pub fn run_primitive_types4() {
    let animals = ['🐪', '🐏'];
    let animal_slice = &animals[1..];

    println!("Array of animals {:?}", animal_slice);
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let num = [1, 2, 3, 4, 5];
        let nice_slice = &num[1..4];

        assert_eq!([2, 3, 4], nice_slice);
    }
}
