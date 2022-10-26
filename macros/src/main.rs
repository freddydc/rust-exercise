mod macros1;
mod macros2;
mod macros3;

use self::macros1::run_macros1;
use self::macros2::run_macros2;
use self::macros3::run_macros3;

fn main() {
    run_macros1();
    run_macros2();
    run_macros3();
}
