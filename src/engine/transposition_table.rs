// third party
use cozy_chess::Move;

// ! Most of this file was created with the assistance of Claude

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Bound {
    Exact, // PV node, score is the true value
    Lower, // fail-high (beta cutoff), true score >= this
    Upper, // fail-low (no move raised alpha), true score <= this
}

#[derive(Clone, Copy)]
pub struct TTEntry {
    pub key: u64,                // zobrist
    pub depth: u8,               // depth of entry
    pub score: i32,              // score of entry
    pub bound: Bound,            // upper/lower bounds of entry (alpha and beta)
    pub best_move: Option<Move>, // best move for this position
    pub age: u8,                 // "older" entries are replaced first 
}

pub struct TranspositionTable {
    entries: Vec<Option<TTEntry>>, // 
    mask: usize,                   // used to convert zobrist into valid indices
}

impl TranspositionTable {
    pub fn new(mb: usize) -> Self {
        let count = (mb * 1024 * 1024) / std::mem::size_of::<Option<TTEntry>>();
        let count = count.next_power_of_two() >> 1; // stay under memory budget
        Self { entries: vec![None; count], mask: count - 1}
    }

    pub fn index(&self, key: u64) -> usize {
        /*
        This is equivalent to key % len(self.entries)
        What we are doing is destroying all bits in the key above the mask
        Conceptually we are handling overflow through wrapping
        e.g. for a 128 entry table, index 129 => index 1
         */
        (key as usize) & self.mask
    }

    pub fn probe(&self, key: u64) -> Option<TTEntry> {
        match self.entries[self.index(key)] {
            Some(e) if e.key == key => Some(e),
            _ => None
        }
    }

    pub fn store(&mut self, new_entry: &TTEntry) {
        let table_index = self.index(new_entry.key);

        // &self.entries[table_index] is an Option<TTEntry>
        // we must handle the case where its occupied and where it isn't
        let replace = match &self.entries[table_index] {
            // an slot is occupied, replace it if the new entry is more recent
            // or if the new entry was found at a greater depth
            Some(existing) => {
                existing.age != new_entry.age || new_entry.depth >= existing.depth
            },
            // the slot is empty, replace it
            None => true,
        };
        if replace {
            self.entries[table_index] = Some(new_entry.clone());
        }
    }
}