mod hello_world;
mod terminal_ferris;
mod variables;
mod array;

use hello_world::{hello_world_test};
use terminal_ferris::{terminal_ferris_test};
use variables::{variables_test};
use array::array_test;

fn main() {
    array_test::run();
}