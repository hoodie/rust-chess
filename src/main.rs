#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

extern crate rand;
extern crate clap;
use clap::{App, SubCommand, Arg};

mod board;
mod gamestate;
mod tests;
mod display;
use gamestate::GameState;


fn run_games(interactive:bool)
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

        if interactive {
            let mut devnull= String::new();
            std::io::stdin().read_line(&mut devnull);
        }
        else{
            std::thread::sleep_ms(150);
        }

    }
}
fn main()
{
    let matches = App::new("Chess")
                        .version("0.0.1")
                        .author("Hendrik Sollich <hendrik@hoodie.de>")
                        .about("tries to play chess")
                        .arg(
                            Arg::with_name("interactive")
                            .short("i")
                            .help("Run step by step interatively instead of automatically")
                            .long("interactive")
                            )
                        .get_matches();


    if  matches.is_present("interactive") {
        println!("Running interactively");
        run_games(true);
    } else{
        println!("Running automatically");
        run_games(false);
    }
}
