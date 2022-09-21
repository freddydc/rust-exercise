// generics1.rs

pub fn run_generics1() {
    let shopping_list: Vec<&str> = vec!["milk", "carrot"];
    println!("{:#?}", shopping_list);
}
