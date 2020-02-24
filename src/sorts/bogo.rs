use crate::SortingAlgorithm;

use piston::input::*;
use rand::{prelude::*, seq::SliceRandom};

impl SortingAlgorithm for BogoSort {
    fn step(&mut self, _args: &UpdateArgs) -> bool {
        let mut rng = thread_rng();
        self.data.shuffle(&mut rng);
        if self.data.is_sorted() {
            self.locked = true;
            true
        } else {
            false
        }
    }

    fn members(&self) -> &Vec<usize> {
        &self.data
    }

    fn is_locked(&self) -> bool {
        self.locked
    }
}

pub struct BogoSort {
    data: Vec<usize>,
    locked: bool,
}

impl BogoSort {
    pub fn new(len: usize) -> Self {
        let mut rng = thread_rng();
        let mut items = (0usize..len).collect::<Vec<usize>>();
        items.shuffle(&mut rng);
        Self {
            data: items,
            locked: false,
        }
    }
}
