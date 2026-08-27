// third party
use cozy_chess::{Board, Move, Piece};

// std
use std::cmp::max;

// project
use crate::engine::transposition_table::{Bound, TTEntry, TranspositionTable};
use crate::engine::evaluation::{INFINITY, MATE_SCORE, Score, evaluate, get_piece_value};
use crate::engine::move_list::MoveList;

#[derive(Clone, Copy)]
struct SearchCTX {
    depth: i32,
    ply: usize,
    alpha: i32,
    beta: i32,
}

pub struct Engine {
    pub nodes_searched: i32,
    pub best_move: Option<Move>,

    tt: TranspositionTable,
    age: u8,
}

impl Engine {
    const MAX_PLY: i32 = 8;

    pub fn new() -> Self {
        Self {
            nodes_searched: 0,
            best_move: None,
            tt: TranspositionTable::new(64),
            age: 0
        }
    }

    pub fn get_best_move(&mut self, board: &Board, max_depth: i32) -> Option<Move> {
        self.age = self.age.wrapping_add(1);
        for depth in 1..(max_depth+1) {
            // debug
            self.nodes_searched = 0;

            // search meta
            self.best_move = None;

            // perform search
            let score = self.alpha_beta(
                &board,
                SearchCTX {
                    depth:  depth,
                    ply:    0,
                    alpha: -INFINITY,
                    beta:   INFINITY,
                }
            );

            // debug
            print!("info depth {} ", depth);
            print!("score cp {} ", score);
            print!("nodes {} ", self.nodes_searched);
            print!("pv {}\n", self.best_move.unwrap()); // ! Could be a problem later
        }

        self.best_move
    }

    fn alpha_beta(&mut self, board: &Board, mut ctx: SearchCTX) -> Score {
        // debug
        self.nodes_searched += 1;

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
        
        // leaf node reached
        if ctx.depth == 0 {
            return self.quiescence_search(
                &board,
                ctx.clone(),
            );
        }

        // get legal moves
        let mut legal_moves = self.get_legal_moves(board);
        legal_moves.sort_by_key(|mv| self.score_move(board, &mv, &tt_move));

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

    fn quiescence_search(&mut self, board: &Board, mut ctx: SearchCTX) -> Score {
        self.nodes_searched += 1;
        
        // prevent depth explosions
        if ctx.ply as i32 > Self::MAX_PLY {
            return evaluate(board);
        }

        // stand pat is not valid in checks
        let is_check = !board.checkers().is_empty();
        let mut best_score = if is_check {
            -INFINITY
        } else {
            let static_score = evaluate(board);

            if static_score >= ctx.beta {
                return ctx.beta;
            }
            ctx.alpha = max(static_score, ctx.alpha);

            static_score
        };

        // get legal moves
        let legal_moves = if is_check {
            self.get_legal_moves(board)
        } else {
            self.get_legal_captures(board)
        };

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

    fn get_legal_moves(&self, board: &Board) -> MoveList {
        let mut legal_moves = MoveList::new();
        board.generate_moves(|move_group| {
            for mv in move_group {
                legal_moves.push(mv);
            }
            return false
        });

        legal_moves
    }

    fn get_legal_captures(&self, board: &Board) -> MoveList {
        let mut legal_captures = MoveList::new();

        let enemy_pieces = board.colors(!board.side_to_move());

        board.generate_moves(|mut move_group| {
            move_group.to &= enemy_pieces;

            for mv in move_group {
                legal_captures.push(mv);
            }
            return false
        });

        legal_captures.sort_by_key(|mv| self.mvv_lva_score(board, &mv));
        legal_captures
    }

    // ! Created by Claude
    fn score_to_tt(&self, score: Score, ply: i32) -> Score {
        if score >= MATE_SCORE - Engine::MAX_PLY { score + ply }
        else if score <= -MATE_SCORE + Engine::MAX_PLY { score - ply }
        else { score }
    }

    // ! Created by Claude
    fn score_from_tt(&self, score: Score, ply: i32) -> Score {
        if score >= MATE_SCORE - Engine::MAX_PLY { score - ply }
        else if score <= -MATE_SCORE + Engine::MAX_PLY { score + ply }
        else { score }
    }

    fn score_move(&self, board: &Board, mv: &Move, tt_move: &Option<Move>) -> i32 {
        if tt_move.is_some() {
            if mv == &tt_move.unwrap() {
                return INFINITY;
            }
        }

        let is_en_passant = board.piece_on(mv.from) == Some(Piece::Pawn)
            && mv.from.file() != mv.to.file()
            && board.piece_on(mv.to).is_none();
    
        // capture moves are given priority while other moves wait
        if board.piece_on(mv.to).is_some() || is_en_passant {
            self.mvv_lva_score(board, mv)
        } else {
            0
        }
    }

    fn mvv_lva_score(&self, board: &Board, mv: &Move) -> i32 {
        let attacker = board
            .piece_on(mv.from)
            .expect("Piece msut exist on from square");
        // if the victim does not exist this move is en passant
        let victim = board
            .piece_on(mv.to)
            .unwrap_or(Piece::Pawn);

        get_piece_value(victim) * 10 - get_piece_value(attacker)
    }
}