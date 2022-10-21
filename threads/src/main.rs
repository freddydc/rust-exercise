mod threads1;
mod threads2;
mod threads3;

use self::threads1::run_threads1;
use self::threads2::run_threads2;
use self::threads3::run_threads3;

fn main() {
    run_threads1();
    run_threads2();
    run_threads3();
}
