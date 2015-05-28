#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]


mod board;
mod gamestate;
use gamestate::{GameState, Move, Point};
mod tests;


fn main()
{
    let mut state = GameState::new();
    state.init_board();
    state.print_board();
    let piece = match state.get_field(Point(1,1)){
        &board::Field::Empty    => Option::None,
        &board::Field::Piece(x) => Option::Some(x),
    };
    println!("{:?}", piece)
}
