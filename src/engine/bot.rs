// third party
use cozy_chess::{Board, Move};
use rand::RngExt;

// project
use super::move_list::MoveList;

pub fn get_best_move(board: &mut Board) -> Move {
    // get legal moves
    let mut move_list = MoveList::new();
    board.generate_moves(|moves| {
        for mv in moves {
            move_list.push(mv);
        }
        return false
    });
    
    // pick random move
    let move_index = rand::rng().random_range(0..move_list.count());
    move_list[move_index].unwrap()
}