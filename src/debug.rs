use cozy_chess::{Board, Piece, Color, Square, File, Rank};

fn ascii_repr(piece: Piece, color: Color) {
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

fn char_repr(piece: Piece, color: Color) {
    if color == Color::White {
        let symbol = match piece {
            Piece::Pawn   => "P",
            Piece::Knight => "N",
            Piece::Bishop => "B",
            Piece::Rook   => "R",
            Piece::Queen  => "Q",
            Piece::King   => "K"
        };
        print!(" {} |", symbol);
    }
    else {
        let symbol = match piece {
            Piece::Pawn   => "p",
            Piece::Knight => "n",
            Piece::Bishop => "b",
            Piece::Rook   => "r",
            Piece::Queen  => "q",
            Piece::King   => "k"
        };
        print!(" {} |", symbol);
    }
}

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
            
            char_repr(piece, color);
        }
    }
    print!("\n  +---+---+---+---+---+---+---+---+\n    a   b   c   d   e   f   g   h\n");
}