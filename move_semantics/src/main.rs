mod move_semantics1;
mod move_semantics2;

use move_semantics1::run_move_semantics1;
use move_semantics2::run_move_semantics2;

fn main() {
    run_move_semantics1();
    run_move_semantics2();
}
