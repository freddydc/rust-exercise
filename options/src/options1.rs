// options1.rs

// This function returns how much ICE CREAM there is left in the fridge.
// If it's before 10PM, there's 5 pieces left. At 10PM, someone eats them
// all, so there'll be no more left.

pub fn run_options1() {
    let ice_creams = maybe_ice_cream(12);
    println!("{:?}", ice_creams);
}

fn maybe_ice_cream(time_of_day: u16) -> Option<u16> {
    // We use the 24-hour system here, so 10PM is a value of 22
    // The Option output should gracefully handle cases where time_of_day > 24.

    match time_of_day {
        0..=21 => Some(5),
        22..=24 => Some(0),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_ice_cream() {
        assert_eq!(maybe_ice_cream(9), Some(5));
        assert_eq!(maybe_ice_cream(10), Some(5));
        assert_eq!(maybe_ice_cream(23), Some(0));
        assert_eq!(maybe_ice_cream(22), Some(0));
        assert_eq!(maybe_ice_cream(25), None);
    }

    #[test]
    fn raw_value() {
        let ice_creams = maybe_ice_cream(12).unwrap();
        assert_eq!(ice_creams, 5);
    }
}
