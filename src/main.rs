mod move_list;
mod debug;

// third party
use cozy_chess::{Board, Move};
use rand::RngExt;

// project
use move_list::MoveList;
use debug::print_board;

// std
use std::io;

fn main() {
    // Start position
    let mut board = Board::default();
    print_board(&board);

    for _ in 0..100 {
        // Todo: Refuse illegal moves
        // Todo: Exit upon game completion
        // Todo: Isolate bot code to separate file

        // get user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let player_move = input.trim_end().parse::<Move>().unwrap();
        board.play(player_move);
        
        // get legal moves
        let mut move_list = MoveList::new();
        board.generate_moves(|moves| {
            for mv in moves {
                move_list.push(mv);
            }
            return false
        });

        // play random move
        let move_index = rand::rng().random_range(0..move_list.count());
        board.play(move_list[move_index].unwrap());
        print_board(&board);
    }
}

/* print moves
for index in 0..(move_list.count()) {
    println!("{}", move_list[index].unwrap());
}
*/