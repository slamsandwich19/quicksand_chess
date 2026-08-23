mod core;
mod debug;

// third party
use cozy_chess::util::parse_san_move;
use cozy_chess::Board;

// project
use core::engine::Engine;
use debug::print_board;

// std
use std::io;

fn main() {
    let mut engine = Engine::new();

    // Start position
    let mut board = Board::default();
    print_board(&board);

    for _ in 0..100 {
        // Todo: Exit upon game completion

        // get user input
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            let player_move = match parse_san_move(&board, &input.trim_end()) {
                Ok(mv) => mv,
                Err(_) => {
                    println!("Unknown move format, please try again");
                    continue;
                }
            };

            match board.try_play(player_move) {
                Ok(_) => break player_move,
                Err(_) => {
                    println!("Illegal move, please try again:");
                    continue;
                }
            }
        };

        // play bot move
        let best_move = engine.get_best_move(&mut board);
        if best_move.is_none() {
            println!("\nNo move found");
            break
        } else {
            board.play_unchecked(best_move.unwrap());
        }

        print_board(&board);

    }
}