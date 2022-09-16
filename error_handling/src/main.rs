mod errors1;
mod errors2;
mod errors3;
mod errors4;

use self::errors1::run_errors1;
use self::errors2::run_errors2;
use self::errors3::run_errors3;
use self::errors4::run_errors4;

fn main() {
    run_errors1();
    run_errors2();
    run_errors3().unwrap();
    run_errors4();
}
