mod debug;

use cozy_chess::{Board, Move};
use debug::print_board;

fn main() {
    // Start position
    let board = Board::default();
    
    print_board(board);
}
