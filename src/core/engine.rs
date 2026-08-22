// third party
use cozy_chess::{Board, Move};

// standard
use std::cmp::max;

// project
use super::move_list::MoveList;
use super::evaluation::evaluate;
use super::super::debug::print_board;

pub struct Engine {
    pub nodes_searched: i32
}

impl Engine {
    pub fn new() -> Self {
        Self { nodes_searched: 0 }
    }

    pub fn get_legal_moves(&self, board: &Board) -> MoveList {
        let mut legal_moves = MoveList::new();
        board.generate_moves(|piece_moves| {
            for mv in piece_moves {
                legal_moves.push(mv);
            }
            return false
        });
        legal_moves
    }

    pub fn get_best_move(&mut self, board: &Board) -> Option<Move> {
        // debug
        self.nodes_searched = 0;

        let legal_moves = self.get_legal_moves(board);
        let mut best_score = -100_000;
        let mut best_move = legal_moves[0];

        for current_move in legal_moves {
            let mut next_board = board.clone();
            next_board.play_unchecked(current_move);
            let score = -self.alpha_beta(&next_board,3, -100000, 100000);

            print!("{}, ", score);

            if score > best_score {
                best_score = score;
                best_move = Some(current_move);
            }
        }

        print!("\ninfo depth 3");
        print!(" score cp {}", best_score);
        print!(" nodes {}", self.nodes_searched);

        best_move
    }

    fn alpha_beta(&mut self, board: &Board, depth: i32, alpha: Score, beta: Score) -> Score {
        if depth == 0 {
            self.nodes_searched += 1;
            return evaluate(&board)
        };

        // get legal moves
        let legal_moves = self.get_legal_moves(board);
        let mut best_score = -100_000;

        // get best move
        for current_move in legal_moves {
            let mut next_board = board.clone();
            next_board.play_unchecked(current_move);
            let score = -self.alpha_beta(&next_board, depth - 1, -beta, -alpha);

            best_score = max(score, best_score);
        }

        best_score
    }
}

type Score = i32;
