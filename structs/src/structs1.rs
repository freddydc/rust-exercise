// structs1.rs

#[derive(Debug)]
struct ColorClassicStruct {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug)]
struct ColorTupleStruct(u8, u8, u8);

#[derive(Debug)]
struct UnitLikeStruct;

pub fn run_structs1() {
    let green1 = ColorClassicStruct {
        red: 0,
        green: 255,
        blue: 0,
    };

    // Patterns to destructure structs.
    let ColorClassicStruct { red, green, blue } = green1;

    let green2 = ColorTupleStruct(red, green, blue);

    println!("{:#?}", green1);
    println!("{:#?}", green2);
    println!("{:?}s are fun!", UnitLikeStruct);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_structs() {
        let green = ColorClassicStruct {
            red: 0,
            green: 255,
            blue: 0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        let green = ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        let unit_like_struct = UnitLikeStruct;
        let message = format!("{:?}s are fun!", unit_like_struct);

        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}
