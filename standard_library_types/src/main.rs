mod box1;
mod iterators1;
mod iterators2;
mod iterators3;
mod iterators4;
mod iterators5;

use self::box1::run_box1;
use self::iterators1::run_iterators1;
use self::iterators2::run_iterators2;
use self::iterators3::run_iterators3;
use self::iterators4::run_iterators4;
use self::iterators5::run_iterators5;

fn main() {
    run_iterators1();
    run_iterators2();
    run_iterators3();
    run_iterators4();
    run_iterators5();
    run_box1();
}