// lifetimes2.rs

pub fn run_lifetimes2() {
    let lamb = String::from("The herd of lambs");
    let result;
    {
        let goat = String::from("The goat");
        result = longest(lamb.as_str(), goat.as_str());
        println!("The longest animal is '{}'", result);
    }
}

fn longest<'a>(animal_one: &'a str, animal_two: &'a str) -> &'a str {
    if animal_one.len() > animal_two.len() {
        animal_one
    } else {
        animal_two
    }
}
