// third party
use cozy_chess::Board;

// project
use crate::engine::move_list::MoveList;

#[derive(Clone, Copy)]
pub struct SearchCTX {
    pub depth: i32,
    pub ply: usize,
    pub alpha: i32,
    pub beta: i32,
}

pub fn get_legal_captures(board: &Board) -> MoveList {
    let mut legal_captures = MoveList::new();

    let enemy_pieces = board.colors(!board.side_to_move());

    board.generate_moves(|mut move_group| {
        move_group.to &= enemy_pieces;

        for mv in move_group {
            legal_captures.push(mv);
        }
        return false
    });

    legal_captures
}

pub fn get_legal_moves(board: &Board) -> MoveList {
    let mut legal_moves = MoveList::new();
    board.generate_moves(|move_group| {
        for mv in move_group {
            legal_moves.push(mv);
        }
        return false
    });

    legal_moves
}