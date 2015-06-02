#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

extern crate rand;
use std::io::{stdin};

mod board;
mod gamestate;
mod tests;
mod display;
use gamestate::GameState;


fn main()
{
    let mut game = GameState::new();
    let mut count = 0;
    let max_count = 1_000;

    loop {
        let len = game.moves[game.current_player()].len();
        if len == 0 {
            game.print_board();
            println!("No more moves for player {:?} after {} rounds", game.current_player().color, count);
            break; }
        if count >= max_count{
            game.print_board();
            println!("Terminated after {} rounds ", count);
            break; }
        let move_choice = rand::random::<usize>() % len;
        game.performe_move(move_choice); count += 1;
        game.print_board();

        let mut devnull= String::new();
        std::io::stdin().read_line(&mut devnull);
        //std::thread::sleep_ms(150);
    }
}
