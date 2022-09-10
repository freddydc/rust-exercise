// options2.rs

pub fn run_options2() {
    let target = "mornings";
    let optional_target = Some(target);

    if let Some(word) = optional_target {
        println!("{}", word);
    }

    let mut optional_integers: Vec<Option<i8>> = (1..=10).into_iter().map(Some).collect();

    while let Some(Some(integer)) = optional_integers.pop() {
        println!("{}", integer);
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn simple_option() {
        let target = "mornings";
        let optional_target = Some(target);

        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let mut range = 10;
        let mut optional_integers: Vec<Option<i8>> = Vec::new();

        for i in 0..(range + 1) {
            optional_integers.push(Some(i));
        }

        while let Some(Some(integer)) = optional_integers.pop() {
            assert_eq!(integer, range);
            range -= 1;
        }
    }
}
