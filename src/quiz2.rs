// quiz2.rs
// This is a quiz for the following sections:
// - Strings
// - Vectors
// - Move semantics
// - Modules
// - Enums

// Let's build a little machine in form of a function.
// As input, we're going to give a list of strings and commands. These commands
// determine what action is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "," + "morning" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.

use self::my_module::transformer;

pub fn run_quiz2() {
    let output = transformer(vec![
        ("hello".into(), Command::Uppercase),
        (" all roads lead to rome! ".into(), Command::Trim),
        ("good".into(), Command::Append(1)),
        ("morning".into(), Command::Append(5)),
    ]);

    println!("{:#?}", output);
}

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        use Command::*;
        let mut output: Vec<String> = vec![];

        for (arg, command) in input.iter() {
            match command {
                Uppercase => output.push(arg.to_uppercase()),
                Trim => output.push(arg.trim().into()),
                Append(n) => {
                    let s = (1..=*n)
                        .into_iter()
                        .fold(arg.to_string(), |acc, _| format!("{},morning", acc));
                    output.push(s);
                }
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn little_machine() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("good".into(), Command::Append(1)),
            ("morning".into(), Command::Append(5)),
        ]);

        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "good,morning");
        assert_eq!(output[3], "morning,morning,morning,morning,morning,morning");
    }
}
