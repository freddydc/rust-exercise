// clippy1.rs

use std::f32;

pub fn run_clippy1() {
    let pi = f32::consts::PI;
    let radius = 5.00_f32;

    let area = pi * f32::powi(radius, 2);

    println!(
        "The area of a circle with radius ({:.2}) is ({:.5})",
        radius, area
    );
}
