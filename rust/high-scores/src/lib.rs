use std::{collections::BinaryHeap, iter};

#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
    heap: BinaryHeap<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            scores: scores.to_vec(),
            heap: BinaryHeap::from(scores.to_owned()),
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.heap.peek().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut heap = self.heap.clone();
        iter::from_fn(move || heap.pop()).take(3).collect()
    }
}
