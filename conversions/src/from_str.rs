// from_str.rs

// Additionally, upon implementing FromStr, you can use the `parse` method
// on strings to generate an object of the implementor type.

use std::num::ParseIntError;
use std::str::FromStr;

pub fn run_from_str() {
    let a = "Goat,10".parse::<Author>().unwrap();
    println!("{:#?}", a);
}

#[derive(Debug, PartialEq)]
struct Author {
    name: String,
    age: usize,
}

#[derive(Debug, PartialEq)]
enum ParsePersonError {
    Empty,
    BadLen,
    NoName,
    ParseInt(ParseIntError),
}

impl From<ParseIntError> for ParsePersonError {
    fn from(err: ParseIntError) -> ParsePersonError {
        ParsePersonError::ParseInt(err)
    }
}

impl FromStr for Author {
    type Err = ParsePersonError;

    fn from_str(s: &str) -> Result<Author, Self::Err> {
        if s.is_empty() {
            return Err(ParsePersonError::Empty);
        }

        let store: Vec<&str> = s.split(',').collect();

        if store.len() != 2 {
            return Err(ParsePersonError::BadLen);
        }

        let name = store[0].to_string();

        if name.is_empty() {
            return Err(ParsePersonError::NoName);
        }

        let age = store[1].parse::<usize>()?;

        Ok(Author { name, age })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Author>(), Err(ParsePersonError::Empty));
    }

    #[test]
    fn good_input() {
        let a = "Bear,32".parse::<Author>();
        assert!(a.is_ok());
        let a = a.unwrap();
        assert_eq!(a.name, "Bear");
        assert_eq!(a.age, 32);
    }

    #[test]
    fn missing_age() {
        assert!(matches!(
            "Bear,".parse::<Author>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!(
            "Bear,twenty".parse::<Author>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("Bear".parse::<Author>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",10".parse::<Author>(), Err(ParsePersonError::NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(
            ",".parse::<Author>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Author>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("Bear,32,".parse::<Author>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!(
            "Bear,32,Camel".parse::<Author>(),
            Err(ParsePersonError::BadLen)
        );
    }
}
