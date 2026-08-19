mod engine;
mod debug;

// third party
use cozy_chess::util::parse_san_move;
use cozy_chess::Board;

// project
use engine::bot::get_best_move;
use debug::print_board;

// std
use std::io;

fn main() {
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
        let best_move = get_best_move(&mut board);
        board.play(best_move);

        // display board
        print_board(&board);
    }
}