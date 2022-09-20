mod errors1;
mod errors2;
mod errors3;
mod errors4;
mod errors5;
mod errors6;

use self::errors1::run_errors1;
use self::errors2::run_errors2;
use self::errors3::run_errors3;
use self::errors4::run_errors4;
use self::errors5::run_errors5;
use self::errors6::run_errors6;

fn main() {
    run_errors1();
    run_errors2();
    run_errors3().unwrap();
    run_errors4();

    if let Err(e) = run_errors5() {
        println!("{}", e);
    }

    run_errors6();
}
