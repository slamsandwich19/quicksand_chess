// third party
use cozy_chess::{Board, Move, Piece};

// project
use super::move_list::MoveList;
use super::evaluation::{evaluate, get_piece_value};

type Score = i32;

pub struct Engine {
    pub nodes_searched: i32
}

impl Engine {
    const CAPTURE_BONUS: i32 = 10_000;

    pub fn new() -> Self {
        Self { nodes_searched: 0 }
    }

    pub fn mvv_lva_score(&self, board: &Board, mv: Move) -> i32 {
        let attacker = board
            .piece_on(mv.from)
            .expect("Piece must exist on from square");
        let victim = board
            .piece_on(mv.to)
            .unwrap_or(Piece::Pawn);

        get_piece_value(victim) * 10 - get_piece_value(attacker)
    }

    pub fn score_move(&self, board: &Board, mv: Move) -> i32 {
        let is_en_passant = board.piece_on(mv.from) == Some(Piece::Pawn)
            && mv.from.file() != mv.to.file()
            && board.piece_on(mv.to).is_none();
    
        if board.piece_on(mv.to).is_some() || is_en_passant {
            Self::CAPTURE_BONUS + self.mvv_lva_score(board, mv)
        } else {
            0
        }
    }

    pub fn get_legal_captures(&self, board: &Board) -> MoveList {
        let mut legal_captures = MoveList::new();

        let enemy_pieces = board.colors(!board.side_to_move());

        board.generate_moves(|mut move_group| {
            move_group.to &= enemy_pieces;

            for mv in move_group {
                legal_captures.push(mv);
            }
            return false
        });

        legal_captures.sort_by_key(|mv| self.mvv_lva_score(board, mv));
        legal_captures
    }

    pub fn get_legal_moves(&self, board: &Board) -> MoveList {
        let mut legal_moves = MoveList::new();
        board.generate_moves(|move_group| {
            for mv in move_group {
                legal_moves.push(mv);
            }
            return false
        });


        legal_moves
    }

    pub fn get_best_move(&mut self, board: &Board) -> Option<Move> {
        // debug
        self.nodes_searched = 0;

        let mut legal_moves = self.get_legal_moves(board);
        legal_moves.sort_by_key(|mv| self.score_move(board, mv));
        let mut best_score = -100_000;
        let mut best_move = legal_moves[0];

        for current_move in legal_moves {
            let mut next_board = board.clone();
            next_board.play_unchecked(current_move);
            let score = -self.alpha_beta(&next_board, 3, -100000, 100000);

            if score > best_score {
                best_score = score;
                best_move = Some(current_move);
            }
        }

        print!("info depth 1 ");
        print!("score cp {} ", best_score);
        print!("nodes {} ", self.nodes_searched);
        print!("pv {}\n", best_move.unwrap());

        print!("info depth 2 ");
        print!("score cp {} ", best_score);
        print!("nodes {} ", self.nodes_searched);
        print!("pv {}\n", best_move.unwrap());
        
        println!("info depth 1 pv {}", best_move.unwrap());

        best_move
    }

    fn alpha_beta(&mut self, board: &Board, depth: i32, mut alpha: Score, beta: Score) -> Score {
        if depth == 0 {
            return self.quiescence_search(&board, alpha, beta, 0);
        };

        // get legal moves
        let mut legal_moves = self.get_legal_moves(board);
        legal_moves.sort_by_key(|mv| self.score_move(board, mv));
        let mut best_score = -100_000;

        if legal_moves.is_empty() {
            if board.checkers().is_empty() {
                return 0;
            } else {
                return -90_000 - depth;
            }
        }

        // get best move
        for current_move in legal_moves {
            let mut next_board = board.clone();
            next_board.play_unchecked(current_move);
            let score = -self.alpha_beta(&next_board, depth - 1, -beta, -alpha);

            if score > best_score {
                best_score = score;
            }

            if score > alpha {
                alpha = score;
            }

            if score >= beta {
                break;
            }
        }

        best_score
    }

    fn quiescence_search(&mut self, board: &Board, mut alpha: i32, beta: i32, qs_depth: i32) -> i32 {
        self.nodes_searched += 1;

        // prevent depth explosions
        if qs_depth > 2 {
            return evaluate(board);
        }

        // stand pat is not valid in checks
        let is_check = !board.checkers().is_empty();
        let best_score = if is_check {
            -100_000
        } else {
            let static_score = evaluate(board);

            if static_score >= beta {
                return beta;
            }
            if static_score > alpha {
                alpha = static_score;
            }

            static_score
        };
        
        let legal_moves = if is_check {
            self.get_legal_moves(board)
        } else {
            self.get_legal_captures(board)
        };
        
        for current_move in legal_moves {
            let mut next_board = board.clone();
            next_board.play_unchecked(current_move);
            let score = -self.quiescence_search(&next_board, -beta, -alpha, qs_depth + 1);

            if score >= beta {
                return score;
            }
            if score > alpha {
                alpha = score;
            }
        }

        best_score
    }
}
