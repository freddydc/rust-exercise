mod from_into;
mod using_as;

use self::from_into::run_from_into;
use self::using_as::run_using_as;

fn main() {
    run_using_as();
    run_from_into();
}
