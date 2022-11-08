// as_ref_mut.rs
// AsRef and AsMut allow for cheap reference-to-reference conversions.

pub fn run_as_ref_mut() {
    let s = String::from("🐪");
    let c1 = byte_counter(s.clone());
    let c2 = char_counter(s);
    let mut num: Box<u32> = Box::new(3);

    num_sq(&mut num);

    println!("Byte is: ({})", c1);
    println!("Char is: ({})", c2);
    println!("Square is: ({})", num);
}

fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().as_bytes().len()
}

fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

fn num_sq<T: AsMut<u32>>(arg: &mut T) {
    *arg.as_mut() *= *arg.as_mut();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "🐪";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Goat";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("🐪");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Goat");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn square_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
