extern crate bowlinggame;
use cucumber::{given, then, when, World};
use bowlinggame::Game;

#[derive(World, Debug, Default)]
pub struct State {
    input: Option<Game>,
    output: Option<i32>,
}

fn main() {
    futures::executor::block_on(State::run("features/"));
}

#[given(expr = "we have a bowling game")]
fn bowling_given(w: &mut State) {
    w.input = Some(Game::new());
}
#[when(expr = "we roll {int} balls of {int} pins")]
fn bowling_when(w: &mut State, num_balls: i32, num_pins: i32) {
    match &mut w.input {
        Some(v) => {
            for _ in 0..num_balls {
                v.roll(num_pins);
            }
            w.output = Some(v.score())
        },
        None => assert!(false),
    };
}
#[then(expr = "we have a total score of {int}")]
fn bowling_then(w: &mut State, expected: i32) {
    match &w.output {
        Some(v) => assert_eq!(v.to_owned(), expected),
        None => assert!(false),
    };
}

#[when(expr = "we roll a strike followed by two {int}s")]
fn bowling_when_strike(w: &mut State, num_pins: i32) {
    match &mut w.input {
        Some(v) => {
            v.roll(10);
            for _ in 0..2 {
                v.roll(num_pins);
            }
            w.output = Some(v.score())
        },
        None => assert!(false),
    };
}

#[when(expr = "we roll a spare followed by a {int}")]
fn bowling_when_spare(w: &mut State, num_pins: i32) {
    match &mut w.input {
        Some(v) => {
            v.roll(5);
            v.roll(5);
            
            v.roll(num_pins);
            w.output = Some(v.score())
        },
        None => assert!(false),
    };
}