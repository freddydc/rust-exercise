mod lifetimes1;
mod lifetimes2;
mod lifetimes3;

use self::lifetimes1::run_lifetimes1;
use self::lifetimes2::run_lifetimes2;
use self::lifetimes3::run_lifetimes3;

fn main() {
    run_lifetimes1();
    run_lifetimes2();
    run_lifetimes3();
}
