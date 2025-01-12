mod hello_world;
mod terminal_ferris;
mod variables;
mod array;
mod number;
mod variables_extra;

use hello_world::{hello_world_test};
use terminal_ferris::{terminal_ferris_test};
use variables::{variables_test};
use array::array_test;
use number::number_test;
use variables_extra::variables_extra_test;

fn main() {
    variables_extra_test::run();
}