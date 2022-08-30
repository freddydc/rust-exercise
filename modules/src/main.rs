mod modules1;
mod modules2;
mod modules3;

use self::modules1::run_modules1;
use self::modules2::run_modules2;
use self::modules3::run_modules3;

fn main() {
    run_modules1();
    run_modules2();
    run_modules3();
}
