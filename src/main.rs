#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

extern crate rand;
use std::io;

mod board;
mod gamestate;
use gamestate::{GameState, Move, Point};
mod tests;


fn main()
{
    let mut state = GameState::new();
    state.init_board();
    state.print_board();
    let piece = match state.get_field(Point{x:1,y:1}){
        &board::Field::Empty    => Option::None,
        &board::Field::Piece(x) => Option::Some(x),
    };

    println!("{:?}", piece);
    loop {
        let moves = state.get_moves(&state.current_player());
        if moves.len() == 0 {println!("no more moves for player {:?}", state.current_player().color);break; }
        let index = rand::random::<usize>() % moves.len();
        let move_choice = &moves[index];
        state.performe_move(move_choice);
        state.print_board();

        let mut devnull= String::new();
        io::stdin().read_line(&mut devnull);
    }
}
