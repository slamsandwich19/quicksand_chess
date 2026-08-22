// third party
use cozy_chess::{Board, Move, Color};

// project
use super::move_list::MoveList;
use super::evaluation::evaluate;

pub fn get_best_move(board: &Board) -> Move {
    println!("{}", evaluate(board));

    // get legal moves
    let mut legal_moves = MoveList::new();
    board.generate_moves(|moves| {
        for mv in moves {
            legal_moves.push(mv);
        }
        return false
    });

    // get best move
    let mut best_score = if board.side_to_move() == Color::White {-100_000} else {100_000};
    let mut best_move = legal_moves[0].unwrap();
    for index in 0..legal_moves.count() {
        let current_move = legal_moves[index].unwrap();
        let mut next_board = board.clone();
        next_board.play_unchecked(current_move);
        let score = evaluate(&next_board);

        print!("{}, ", score);
        
        if board.side_to_move() == Color::White {
            if score > best_score {
                best_score = score;
                best_move = current_move;
            }
        }
        else  {
            if score < best_score {
                best_score = score;
                best_move = current_move;
            }
        }
        
    }

    best_move
}