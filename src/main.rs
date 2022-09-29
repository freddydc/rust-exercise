mod quiz1;
mod quiz2;
mod quiz3;

use self::quiz2::run_quiz2;
use self::quiz3::run_quiz3;
use quiz1::run_quiz1;

fn main() {
    run_quiz1();
    run_quiz2();
    run_quiz3();
}
