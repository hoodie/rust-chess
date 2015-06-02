#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

extern crate rand;
use std::io::stdin;

mod board;
mod gamestate;
mod tests;
mod display;
use gamestate::GameState;

#[test]
fn it_works() {
    let mut game = GameState::new();
}
