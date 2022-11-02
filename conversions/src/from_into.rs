// from_into.rs

// The From trait is used for value-to-value conversions.
// If From is implemented correctly for a type, the Into trait should work conversely.

pub fn run_from_into() {
    // Use the `from` function
    let p1 = Person::from("Deer,20");
    // Since From is implemented for Person, we should be able to use Into
    let p2: Person = "Bear,70".into();

    println!("{:#?}", p1);
    println!("{:#?}", p2);
}

#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub age: usize,
}

impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("Goat"),
            age: 30,
        }
    }
}

impl From<&str> for Person {
    fn from(s: &str) -> Person {
        if s.is_empty() {
            return Person::default();
        }

        let store = s.splitn(2, ',').collect::<Vec<&str>>();
        let name = store[0].to_string();

        if name.is_empty() {
            return Person::default();
        }

        if let Some(value) = store.get(1) {
            if let Ok(age) = value.parse::<usize>() {
                return Person { name, age };
            }
        }

        Person::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_person() {
        // Test that the default person is 30 year old Goat
        let dp = Person::default();
        assert_eq!(dp.name, "Goat");
        assert_eq!(dp.age, 30);
    }

    #[test]
    fn bad_convert() {
        // Test that Goat is returned when bad string is provided
        let p = Person::from("");
        assert_eq!(p.name, "Goat");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn good_convert() {
        // Test that "Camel,20" works
        let p = Person::from("Camel,20");
        assert_eq!(p.name, "Camel");
        assert_eq!(p.age, 20);
    }

    #[test]
    fn bad_age() {
        // Test that "Camel,twenty" will return the default person due to an error in parsing age
        let p = Person::from("Camel,twenty");
        assert_eq!(p.name, "Goat");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn missing_comma_and_age() {
        let p: Person = Person::from("Camel");
        assert_eq!(p.name, "Goat");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn missing_age() {
        let p: Person = Person::from("Camel,");
        assert_eq!(p.name, "Goat");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn missing_name() {
        let p: Person = Person::from(",10");
        assert_eq!(p.name, "Goat");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "Goat");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "Goat");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn trailing_comma() {
        let p: Person = Person::from("Bear,32,");
        assert_eq!(p.name, "Goat");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn trailing_comma_and_some_string() {
        let p: Person = Person::from("Bear,32,Deer");
        assert_eq!(p.name, "Goat");
        assert_eq!(p.age, 30);
    }
}
