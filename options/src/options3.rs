// options3.rs

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

pub fn run_options3() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{}", p.x, p.y),
        _ => println!("no match"),
    }

    println!("{:#?}", y);
}
