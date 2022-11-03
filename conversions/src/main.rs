mod from_into;
mod from_str;
mod using_as;

use self::from_into::run_from_into;
use self::from_str::run_from_str;
use self::using_as::run_using_as;

fn main() {
    run_using_as();
    run_from_into();
    run_from_str();
}
