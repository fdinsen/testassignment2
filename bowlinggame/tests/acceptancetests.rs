extern crate bowlinggame;
use cucumber::{given, then, when, World};

#[derive(World, Debug, Default)]
pub struct State {
    input: Option<String>,
    output: Option<String>,
}

fn main() {
    futures::executor::block_on(State::run("features/"));
}