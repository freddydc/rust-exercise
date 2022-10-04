// lifetimes3.rs

struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

pub fn run_lifetimes3() {
    let name = String::from("Sheep");
    let title = String::from("The herd of lambs");
    let book = Book {
        author: &name,
        title: &title,
    };

    println!("{} by '{}'", book.title, book.author);
}
