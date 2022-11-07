// try_from_into.rs

// TryFrom is a simple and safe type conversion that may fail in a controlled way under some circumstances.
// Basically, this is the same as From. The main difference is that this should return a Result type
// instead of the target type itself.

use std::convert::{TryFrom, TryInto};

pub fn run_try_from_into() {
    // Use the `try_from` function
    let c1 = Color::try_from((183, 65, 14));

    // Since TryFrom is implemented for Color, we should be able to use TryInto
    let c2: Result<Color, _> = [183, 65, 14].try_into();

    let v = vec![183, 65, 14];

    // With slice we should use `try_from` function
    let c3 = Color::try_from(&v[..]);

    // or take slice within round brackets and use TryInto
    let c4: Result<Color, _> = (&v[..]).try_into();

    println!("Avocado: {:#?}", c1);
    println!("Banana: {:#?}", c2);
    println!("Cherry: {:#?}", c3);
    println!("Pear: {:#?}", c4);
}

#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug, PartialEq)]
enum IntoColorError {
    BadLen,
    IntConversion,
}

// Tuple implementation
impl TryFrom<(i16, i16, i16)> for Color {
    type Error = IntoColorError;

    fn try_from(tuple: (i16, i16, i16)) -> Result<Color, Self::Error> {
        match tuple {
            (r, g, b)
                if !(0..=255).contains(&r) | !(0..=255).contains(&g) | !(0..=255).contains(&b) =>
            {
                Err(IntoColorError::IntConversion)
            }
            (red, green, blue) => Ok(Color {
                red: red as u8,
                green: green as u8,
                blue: blue as u8,
            }),
        }
    }
}

// Array implementation
impl TryFrom<[i16; 3]> for Color {
    type Error = IntoColorError;

    fn try_from(arr: [i16; 3]) -> Result<Color, Self::Error> {
        (&arr[..]).try_into()
    }
}

// Slice implementation
impl TryFrom<&[i16]> for Color {
    type Error = IntoColorError;

    fn try_from(slice: &[i16]) -> Result<Color, Self::Error> {
        if slice.len() != 3 {
            return Err(IntoColorError::BadLen);
        }

        (slice[0], slice[1], slice[2]).try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_out_of_range_positive() {
        assert_eq!(
            Color::try_from((256, 1000, 10000)),
            Err(IntoColorError::IntConversion)
        );
    }

    #[test]
    fn tuple_out_of_range_negative() {
        assert_eq!(
            Color::try_from((-1, -10, -256)),
            Err(IntoColorError::IntConversion)
        );
    }

    #[test]
    fn tuple_sum() {
        assert_eq!(
            Color::try_from((-1, 255, 255)),
            Err(IntoColorError::IntConversion)
        );
    }

    #[test]
    fn tuple_correct() {
        let c: Result<Color, _> = (183, 65, 14).try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }

    #[test]
    fn array_out_of_range_positive() {
        let c: Result<Color, _> = [1000, 10000, 256].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }

    #[test]
    fn array_out_of_range_negative() {
        let c: Result<Color, _> = [-10, -256, -1].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }

    #[test]
    fn array_sum() {
        let c: Result<Color, _> = [-1, 255, 255].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }

    #[test]
    fn array_correct() {
        let c: Result<Color, _> = [183, 65, 14].try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }

    #[test]
    fn slice_out_of_range_positive() {
        let arr = [10000, 256, 1000];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }

    #[test]
    fn slice_out_of_range_negative() {
        let arr = [-256, -1, -10];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }

    #[test]
    fn slice_sum() {
        let arr = [-1, 255, 255];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }

    #[test]
    fn slice_correct() {
        let v = vec![183, 65, 14];
        let c: Result<Color, _> = Color::try_from(&v[..]);
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }

    #[test]
    fn slice_excess_length() {
        let v = vec![0, 0, 0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(IntoColorError::BadLen));
    }

    #[test]
    fn slice_insufficient_length() {
        let v = vec![0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(IntoColorError::BadLen));
    }
}
