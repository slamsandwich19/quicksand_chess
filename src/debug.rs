use cozy_chess::{Board, Piece, Color, Square, File, Rank};

pub fn print_board(board: &Board) {

    for rank in Rank::ALL.into_iter().rev() {
        // display rank header
        print!("\n  +---+---+---+---+---+---+---+---+\n{} |", rank);
        for file in File::ALL {
            let square = Square::new(file, rank);
            let piece = board.piece_on(square);
            let color = board.color_on(square);

            if piece.is_none() {
                print!("   |");
                continue;
            }

            let piece = piece.unwrap();
            let color = color.unwrap();
            
            if color == Color::White {
                let symbol = match piece {
                    Piece::Pawn   => "♟",
                    Piece::Knight => "♞",
                    Piece::Bishop => "♝",
                    Piece::Rook   => "♜",
                    Piece::Queen  => "♛",
                    Piece::King   => "♚"
                };
                print!(" {} |", symbol);
            }
            else {
                let symbol = match piece {
                    Piece::Pawn   => "♙",
                    Piece::Knight => "♘",
                    Piece::Bishop => "♗",
                    Piece::Rook   => "♖",
                    Piece::Queen  => "♕",
                    Piece::King   => "♔"
                };
                print!(" {} |", symbol);
            }
        }
    }
    print!("\n  +---+---+---+---+---+---+---+---+\n    a   b   c   d   e   f   g   h\n");
}