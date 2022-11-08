mod as_ref_mut;
mod from_into;
mod from_str;
mod try_from_into;
mod using_as;

use self::as_ref_mut::run_as_ref_mut;
use self::from_into::run_from_into;
use self::from_str::run_from_str;
use self::try_from_into::run_try_from_into;
use self::using_as::run_using_as;

fn main() {
    run_using_as();
    run_from_into();
    run_from_str();
    run_try_from_into();
    run_as_ref_mut();
}
