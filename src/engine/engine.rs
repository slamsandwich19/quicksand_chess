// third party
use cozy_chess::{Board, Move};

// std
use std::cmp::max;

// project
use crate::engine::transposition_table::{Bound, TTEntry, TranspositionTable};
use crate::engine::evaluation::{INFINITY, MATE_SCORE, evaluate, score_move};
use crate::engine::utils::{SearchCTX, get_legal_moves, get_legal_captures};

pub struct Engine {
    best_move: Option<Move>,
    nodes: i32,
    tt: TranspositionTable,
    age: u8,
}

impl Engine {
    const MAX_PLY: i32 = 8;
    const WINDOW: i32 = 20;

    pub fn new() -> Self {
        Self {
            best_move: None,
            nodes: 0,
            tt: TranspositionTable::new(64),
            age: 0
        }
    }

    pub fn get_best_move(&mut self, board: &Board, max_depth: i32) -> Option<Move> {
        // search meta
        self.best_move = None;
        self.age = self.age.wrapping_add(1);
        let mut score = 0;
        
        for depth in 1..(max_depth+1) {
            // debug
            self.nodes = 0;

            let (mut alpha, mut beta) = if depth >= 3 {
                (score - Self::WINDOW, score + Self::WINDOW)
            } else {
                (-INFINITY, INFINITY)
            };

            let mut search_window = true;
            while search_window {
                // perform search
                score = self.alpha_beta(
                    &board,
                    SearchCTX {
                        depth: depth,
                        ply:   0,
                        alpha: alpha,
                        beta:  beta,
                    }
                );
            
                // true score is lower than alpha (lower bound), widen and try again
                if score <= alpha {
                    alpha = -INFINITY;
                } else if score >= beta {
                    beta = INFINITY;
                } else {
                    search_window = false;
                }
            }

            // debug
            print!("info depth {} ", depth);
            print!("score cp {} ", score);
            print!("nodes {} ", self.nodes);
            print!("pv {}\n", self.best_move.unwrap()); // ! Could be a problem later
        }

        self.best_move
    }

    fn alpha_beta(&mut self, board: &Board, mut ctx: SearchCTX) -> i32 {
        // debug
        self.nodes += 1;

        // probe transposition table
        let key = board.hash();
        let mut tt_move: Option<Move> = None;

        if let Some(entry) = self.tt.probe(key) {
            tt_move = entry.best_move.clone();
            if entry.depth as i32 >= ctx.depth && ctx.ply > 0 {
                let score = self.score_from_tt(entry.score, ctx.ply as i32);
                match entry.bound {
                    Bound::Exact => return score,
                    Bound::Lower if score >= ctx.beta => return score,
                    Bound::Upper if score <= ctx.alpha => return score,
                    _ => {}
                }
            }
        }
        
        // leaf => return eval
        if ctx.depth == 0 {
            return self.quiescence_search(
                &board,
                ctx.clone(),
            );
        }

        // nmp
        let is_check = !board.checkers().is_empty();
        if !is_check && ctx.depth >= 3 {
            let next_board = board.null_move();
            if next_board.is_some() {
                let null_score = -self.alpha_beta(
                    &next_board.unwrap(),
                    SearchCTX { 
                        depth:  ctx.depth - 3,
                        ply:    ctx.ply + 1,
                        alpha: -ctx.beta,
                        beta:  -ctx.beta + 1,
                    }
                );

                if null_score >= ctx.beta {
                    return ctx.beta
                }
            }
        }

        // get legal moves
        let mut legal_moves = get_legal_moves(board);
        legal_moves.sort_by_key(|mv| score_move(board, &mv, &tt_move));

        // check if game is over
        if legal_moves.is_empty() {
            // no checks => stalemate
            if board.checkers().is_empty() {
                return 0;
            // checks => checkmate
            } else {
                return -MATE_SCORE + ctx.ply as i32;
            }
        }

        // search meta
        let original_alpha = ctx.alpha;
        let mut best_score = -INFINITY;
        let mut best_move = None;

        // search moves
        for current_move in legal_moves {
            // clone structs for purity
            let mut next_board = board.clone();
            
            // play move
            next_board.play_unchecked(current_move);
            
            // score move recursively
            let score = -self.alpha_beta(
                &next_board,
                SearchCTX {
                    depth:  ctx.depth - 1,
                    ply:    ctx.ply   + 1,
                    alpha: -ctx.beta,
                    beta:  -ctx.alpha,
                }
            );

            // store search meta and prune
            if score > best_score {
                best_score = score;
                best_move = Some(current_move);
                if ctx.ply == 0 {
                    self.best_move = Some(current_move);
                }
            }

            ctx.alpha = max(score, ctx.alpha);
            if score >= ctx.beta {
                break
            };
        }

        let bound = if best_score <= original_alpha {
            Bound::Upper
        } else if best_score >= ctx.beta {
            Bound::Lower
        } else {
            Bound::Exact
        };

        self.tt.store(&TTEntry {
            key: board.hash(),
            depth: ctx.depth as u8,
            score: self.score_to_tt(best_score, ctx.ply as i32),
            bound,
            best_move,
            age: self.age,
        });

        best_score
    }

    fn quiescence_search(&mut self, board: &Board, mut ctx: SearchCTX) -> i32 {
        self.nodes += 1;
        
        // prevent depth explosions
        if ctx.ply as i32 > Self::MAX_PLY {
            return evaluate(board);
        }

        // stand pat is not valid in checks
        let is_check = !board.checkers().is_empty();
        let mut best_score = if is_check {
            -MATE_SCORE + ctx.ply as i32
        } else {
            let static_score = evaluate(board);

            if static_score >= ctx.beta {
                return static_score;
            }
            ctx.alpha = max(static_score, ctx.alpha);

            static_score
        };

        // get legal moves
        let mut legal_moves = if is_check {
            get_legal_moves(board)
        } else {
            get_legal_captures(board)
        };
        legal_moves.sort_by_key(|mv| score_move(board, &mv, &None));

        // search
        for current_move in legal_moves {
            // clone structs for purity
            let mut next_board = board.clone();

            // play move
            next_board.play_unchecked(current_move);

            // score move recursively
            let score = -self.quiescence_search(
                &next_board,
                SearchCTX {
                    depth:  ctx.depth - 1, 
                    ply:    ctx.ply + 1,
                    alpha: -ctx.beta,
                    beta:  -ctx.alpha,
                }
            );

            // store search meta and prune
            best_score = max(score, best_score);

            if score >= ctx.beta {
                return score;
            }

            ctx.alpha = max(score, ctx.alpha);
        }

        best_score
    }

    // ! Created by Claude
    fn score_to_tt(&self, score: i32, ply: i32) -> i32 {
        if score >= MATE_SCORE - Self::MAX_PLY { score + ply }
        else if score <= -MATE_SCORE + Self::MAX_PLY { score - ply }
        else { score }
    }

    // ! Created by Claude
    fn score_from_tt(&self, score: i32, ply: i32) -> i32 {
        if score >= MATE_SCORE - Self::MAX_PLY { score - ply }
        else if score <= -MATE_SCORE + Self::MAX_PLY { score + ply }
        else { score }
    }
}