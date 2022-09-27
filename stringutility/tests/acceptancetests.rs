extern crate stringutility;
use cucumber::{given, then, when, World};
use stringutility::reverse_str;

#[derive(World, Debug, Default)]
pub struct State {
    input: Option<String>,
    output: Option<String>,
}

fn main() {
    futures::executor::block_on(State::run("features/"));
}

#[given(expr = "we want the reverse of the string {word}")]
fn reverse_given(w: &mut State, temp: String) {
    w.input = Some(temp);
}
#[when(expr = "we input it in the reverse method")]
fn reverse_when(w: &mut State) {
    match &w.input {
        Some(v) => w.output = Some(reverse_str(&v)),
        None => assert!(false),
    };
}
#[then(expr = "we get {word}")]
fn reverse_then(w: &mut State, expected: String) {
    match &w.output {
        Some(v) => assert_eq!(v.to_owned(), expected),
        None => assert!(false),
    };
}
