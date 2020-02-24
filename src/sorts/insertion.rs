use crate::SortingAlgorithm;

use piston::input::*;
use rand::{prelude::*, seq::SliceRandom};

impl SortingAlgorithm for InsertionSort {
    fn step(&mut self, _args: &UpdateArgs) -> bool {
        // println!("Next Step: {} - {:#?}", self.looking_from, self.data);
        let key = self.data[self.looking_from];
        let mut j = self.looking_from;
        while j > 0 && key < self.data[j - 1] {
            self.data[j] = self.data[j - 1];
            j -= 1;
        }
        self.data[j] = key;
        self.looking_from += 1;

        if self.looking_from == self.data.len() {
            self.locked = true;
        }

        true
    }

    fn members(&self) -> &Vec<usize> {
        &self.data
    }

    fn is_locked(&self) -> bool {
        self.locked
    }
}

pub struct InsertionSort {
    pub data: Vec<usize>,
    pub locked: bool,
    pub looking_from: usize,
}

impl InsertionSort {
    pub fn new(len: usize) -> Self {
        let mut rng = thread_rng();
        let mut items = (0usize..len).collect::<Vec<usize>>();
        items.shuffle(&mut rng);
        Self {
            data: items,
            locked: false,
            looking_from: 1,
        }
    }
}
