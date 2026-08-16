use cozy_chess::{Board};

pub fn print_board(board: Board) {
    for rank in 0..8 {
        print!("\n  +---+---+---+---+---+---+---+---+\n{} |", 8 - rank);
        for file in 0..8 {
            print!(" . |");
        }
    }
    print!("\n  +---+---+---+---+---+---+---+---+\n");
}