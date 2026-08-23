// project
use cozy_chess::Move;

// std
use std::ops::Index;

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

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn sort_by_key<F, K>(&mut self, mut key_fn: F)
    where
        F: FnMut(Move) -> K,
        K: Ord,
    {
        self.moves[0..self.count].sort_by_key(|mv| {
            std::cmp::Reverse(key_fn(mv.unwrap()))
        })
    }
}

impl Index<usize> for MoveList {
    type Output = Option<Move>;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.moves[index]
    }
}

// custom iterator struct to keep track of index
pub struct MoveListIntoIter {
    list: MoveList,
    index: usize,
}

impl Iterator for MoveListIntoIter {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.list.count {
            let result = self.list.moves[self.index];
            self.index += 1;
            result
        } else {
            None
        }
    }
}

impl IntoIterator for MoveList {
    type Item = Move;
    type IntoIter = MoveListIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        MoveListIntoIter {
            list: self,
            index: 0,
        }
    }
}