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
            
            let symbol = match piece {
                Piece::Pawn => "P",
                Piece::Knight => "N",
                Piece::Bishop => "B",
                Piece::Rook => "R",
                Piece::Queen => "Q",
                Piece::King => "K"
            };

            let display_char = if color == Color::White {symbol.to_ascii_lowercase()} else {symbol.to_ascii_lowercase()};

            print!(" {} |", display_char);
        }
    }
    print!("\n  +---+---+---+---+---+---+---+---+\n    a   b   c   d   e   f   g   h\n");
}