#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

extern crate rand;
use std::io::{stdin};

mod board;
mod gamestate;
mod tests;
use gamestate::GameState;


fn main()
{
    let mut game = GameState::new();
    game.init_board();
    let mut count = 0;
    let max_count = 1_000;

    loop {
        let moves = game.get_moves(&game.current_player());
        if moves.len() == 0 {
            game.print_board();
            println!("No more moves for player {:?} after {} rounds", game.current_player().color, count);
            break; }
        if count >= max_count{
            game.print_board();
            println!("Terminated after {} rounds ", count);
            break; }

        let move_choice = &moves[rand::random::<usize>() % moves.len()];
        game.performe_move(move_choice); count += 1;
        game.print_board();

        //let mut devnull= String::new();
        //io::stdin().read_line(&mut devnull);
        std::thread::sleep_ms(150);
    }
}
