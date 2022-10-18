mod arc1;
mod box1;
mod cow1;
mod iterators1;
mod iterators2;
mod iterators3;
mod iterators4;
mod iterators5;
mod rc1;

use self::arc1::run_arc1;
use self::box1::run_box1;
use self::cow1::run_cow1;
use self::iterators1::run_iterators1;
use self::iterators2::run_iterators2;
use self::iterators3::run_iterators3;
use self::iterators4::run_iterators4;
use self::iterators5::run_iterators5;
use self::rc1::run_rc1;

fn main() {
    run_iterators1();
    run_iterators2();
    run_iterators3();
    run_iterators4();
    run_iterators5();
    run_box1();
    run_arc1();
    run_rc1();
    run_cow1();
}
