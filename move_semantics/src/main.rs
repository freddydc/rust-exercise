mod move_semantics1;
mod move_semantics2;
mod move_semantics3;
mod move_semantics4;
mod move_semantics5;

use move_semantics1::run_move_semantics1;
use move_semantics2::run_move_semantics2;
use move_semantics3::run_move_semantics3;
use move_semantics4::run_move_semantics4;
use move_semantics5::run_move_semantics5;

fn main() {
    run_move_semantics1();
    run_move_semantics2();
    run_move_semantics3();
    run_move_semantics4();
    run_move_semantics5();
}
