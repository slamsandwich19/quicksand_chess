// project
use cozy_chess::Move;

// std
use std::ops::{Index};

#[derive(Debug, Clone, Copy)]
pub struct MoveList {
    moves: [Option<Move>; 256],
    count: usize,
}

impl MoveList {
    pub fn new() -> Self {
        return MoveList {
            moves: [None; 256],
            count: 0
        }
    }

    pub fn push(&mut self, mv: Move) {
        self.moves[self.count] = Some(mv);
        self.count += 1;
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

impl Index<usize> for MoveList {
    type Output = Option<Move>;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.moves[index]
    }
}