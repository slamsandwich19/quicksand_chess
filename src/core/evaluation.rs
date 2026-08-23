// third party
use cozy_chess::{Piece, Color, Board};

const PIECE_VALUES: [i32; 6] = [100, 320, 330, 500, 900, 0];

pub fn evaluate(board: &Board) -> i32 {
    let mut score = 0;

    for color in Color::ALL {
        for piece in Piece::ALL {
            let pieces = board.colored_pieces(color, piece);

            if color == Color::White {
                score += PIECE_VALUES[piece as usize] * pieces.len() as i32;
            }
            else
            {
                score -= PIECE_VALUES[piece as usize] * pieces.len() as i32;
            }
        }
    }

    if board.side_to_move() == Color::Black {
        score = -score;
    }

    score
}