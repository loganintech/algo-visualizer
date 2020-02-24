use crate::SortingAlgorithm;

use piston::input::*;
use rand::{prelude::*, seq::SliceRandom};

impl SortingAlgorithm for BubbleSort {
    fn step(&mut self, _args: &UpdateArgs) -> bool {
        // println!("Next Step: {} - {:#?}", self.looking_from, self.data);

        // for _ in self.looking_from..(self.data.len() - 1) {}

        for j in 0..(self.data.len() - self.looking_from) {
            if self.data[j] > self.data[j + 1] {
                let temp = self.data[j];
                self.data[j] = self.data[j + 1];
                self.data[j + 1] = temp;
            }
        }

        self.looking_from += 1;

        if self.looking_from - 1 == self.data.len() {
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

pub struct BubbleSort {
    pub data: Vec<usize>,
    pub locked: bool,
    pub looking_from: usize,
}

impl BubbleSort {
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
