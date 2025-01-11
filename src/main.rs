mod hello_world;
mod terminal_ferris;
mod variables;

use hello_world::{hello_world_test};
use terminal_ferris::{terminal_ferris_test};
use variables::{variables_test};

fn main() {
    hello_world_test::run();
    terminal_ferris_test::run();
}