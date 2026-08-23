// third party
use cozy_chess::{Piece, Color, Board};
pub fn get_piece_value(piece: Piece) -> i32 {
    match piece {
        Piece::Pawn => 100,
        Piece::Knight => 320,
        Piece::Bishop => 330,
        Piece::Rook => 500,
        Piece::Queen => 900,
        Piece::King => 0,
    }
}

pub fn evaluate(board: &Board) -> i32 {
    let mut score = 0;

    for color in Color::ALL {
        for piece in Piece::ALL {
            let pieces = board.colored_pieces(color, piece);

            if color == Color::White {
                score += get_piece_value(piece) * pieces.len() as i32;
            }
            else
            {
                score -= get_piece_value(piece) * pieces.len() as i32;
            }
        }
    }

    if board.side_to_move() == Color::Black {
        score = -score;
    }

    score
}