#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

extern crate rand;
extern crate clap;
use clap::{App, SubCommand};

mod board;
mod gamestate;
mod tests;
mod display;
use gamestate::GameState;


fn run_games()
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
fn main()
{
    let matches = App::new("Chess")
                        .version("0.0.1")
                        .author("Hendrik Sollich <hendrik@hoodie.de>")
                        .about("tries to play chess")
                        .args_from_usage("-i --interactive 'Run Step by Step'")
                        .get_matches();


    if let Some(i) = matches.value_of("iteractive") {
        println!("Running Interactively");
    }
    run_games();
}
