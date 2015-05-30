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
    let mut game = GameState::new();
    game.init_board();
    //game.print_board();
    let mut count = 0;
    let max_count = 1_000_000;

    loop {
        let moves = game.get_moves(&game.current_player());
        if moves.len() == 0 {
            println!("after {}, no more moves for player {:?}", count, game.current_player().color);
            game.print_board();
            break; }
        if count >= max_count{
            println!("Terminated after {} rounds ", count);
            game.print_board();
            break; }
        if count % 10_000 == 0{ println!("Warning! {}k rounds already", count/1000); }
        let index = rand::random::<usize>() % moves.len();
        let move_choice = &moves[index];
        game.performe_move(move_choice);
        count += 1;
        //std::thread::sleep_ms(200);
        game.print_board();
        //let mut devnull= String::new();
        //io::stdin().read_line(&mut devnull);
    }
}
