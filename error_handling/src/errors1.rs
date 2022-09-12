// errors1.rs
// This function refuses to generate text to be printed on a name tag if
// you pass it an empty string.

pub fn run_errors1() {
    let data = generate_name_tag_text("Echo".into());

    match data {
        Ok(text) => println!("{}", text),
        Err(e) => println!("{}", e),
    }
}

pub fn generate_name_tag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        Err("`name` was empty; it must be nonempty.".into())
    } else {
        Ok(format!("Hi! My name is {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_name_tag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_name_tag_text("Echo".into()),
            Ok("Hi! My name is Echo".into())
        );
    }

    #[test]
    fn explains_why_generating_name_tag_text_fails() {
        assert_eq!(
            generate_name_tag_text("".into()),
            Err("`name` was empty; it must be nonempty.".into())
        );
    }
}
