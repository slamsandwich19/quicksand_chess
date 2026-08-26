// third party
use cozy_chess::{Piece, Color, Square, Board};

pub type Score = i32;

pub const INFINITY: i32 = 32_001;
pub const MATE_SCORE: i32 = 32_000;

const MG_VALUE: [i32; 6] = [82, 337, 365, 477, 1025, 0];

const MG_PAWN_TABLE: [i32; 64] = [
     0,  0,  0,  0,  0,  0,  0,  0,
    50, 50, 50, 50, 50, 50, 50, 50,
    10, 10, 20, 30, 30, 20, 10, 10,
     5,  5, 10, 25, 25, 10,  5,  5,
     0,  0,  0, 20, 20,  0,  0,  0,
     5, -5,-10,  0,  0,-10, -5,  5,
     5, 10, 10,-20,-20, 10, 10,  5,
     0,  0,  0,  0,  0,  0,  0,  0,
];

const MG_KNIGHT_TABLE: [i32; 64] = [
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -30,  0, 10, 15, 15, 10,  0,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  0, 15, 20, 20, 15,  0,-30,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50,
];

const MG_BISHOP_TABLE: [i32; 64] = [
    -20,-10,-10,-10,-10,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5, 10, 10,  5,  0,-10,
    -10,  5,  5, 10, 10,  5,  5,-10,
    -10,  0, 10, 10, 10, 10,  0,-10,
    -10, 10, 10, 10, 10, 10, 10,-10,
    -10,  5,  0,  0,  0,  0,  5,-10,
    -20,-10,-10,-10,-10,-10,-10,-20,
];

const MG_ROOK_TABLE: [i32; 64] = [
      0,  0,  0,  0,  0,  0,  0,  0,
      5, 10, 10, 10, 10, 10, 10,  5,
     -5,  0,  0,  0,  0,  0,  0, -5,
     -5,  0,  0,  0,  0,  0,  0, -5,
     -5,  0,  0,  0,  0,  0,  0, -5,
     -5,  0,  0,  0,  0,  0,  0, -5,
     -5,  0,  0,  0,  0,  0,  0, -5,
      0,  0,  0,  5,  5,  0,  0,  0,
];

const MG_QUEEN_TABLE: [i32; 64] = [
    -20,-10,-10, -5, -5,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5,  5,  5,  5,  0,-10,
     -5,  0,  5,  5,  5,  5,  0, -5,
      0,  0,  5,  5,  5,  5,  0, -5,
    -10,  5,  5,  5,  5,  5,  0,-10,
    -10,  0,  5,  0,  0,  0,  0,-10,
    -20,-10,-10, -5, -5,-10,-10,-20,
];

const MG_KING_TABLE: [i32; 64] = [
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -20,-30,-30,-40,-40,-30,-30,-20,
    -10,-20,-20,-20,-20,-20,-20,-10,
     20, 20,  0,  0,  0,  0, 20, 20,
     20, 30, 10,  0,  0, 10, 30, 20,
];

pub fn get_piece_value(piece: Piece) -> i32 {
    MG_VALUE[piece as usize]
}

const fn merge_scores(value: i32, table: [i32; 64]) -> [i32; 64] {
    let mut merged = [0i32; 64];
    let mut i = 0;
    while i < 64 {
        merged[i] = value + table[i];
        i += 1;
    }
    merged
}

const MG_TABLES: [[i32; 64]; 6] = [
    merge_scores(MG_VALUE[0], MG_PAWN_TABLE),
    merge_scores(MG_VALUE[1], MG_KNIGHT_TABLE),
    merge_scores(MG_VALUE[2], MG_BISHOP_TABLE),
    merge_scores(MG_VALUE[3], MG_ROOK_TABLE),
    merge_scores(MG_VALUE[4], MG_QUEEN_TABLE),
    merge_scores(MG_VALUE[5], MG_KING_TABLE),
];

fn pst_index(square: Square, color: Color) -> usize {
    match color {
        Color::White => square.flip_rank() as usize,
        Color::Black => square as usize,
    }
}

pub fn evaluate(board: &Board) -> i32 {
    let mut mg_score = 0;

    for color in Color::ALL {
        let sign = if color == Color::White {1} else {-1};

        for piece in Piece::ALL {
            let piece_index = piece as usize;
            let squares = board.colored_pieces(color, piece);

            for square in squares {
                let idx = pst_index(square, color);
                mg_score += sign * MG_TABLES[piece_index][idx];
            }
        }
    }

    let mut score = mg_score;

    if board.side_to_move() == Color::Black {
        score = -score;
    }

    score
}