mod clippy1;
mod clippy2;
mod clippy3;

use self::clippy1::run_clippy1;
use self::clippy2::run_clippy2;
use self::clippy3::run_clippy3;

fn main() {
    run_clippy1();
    run_clippy2();
    run_clippy3();
}
