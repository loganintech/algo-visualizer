use crate::SortingAlgorithm;

use piston::input::*;
use rand::{prelude::*, seq::SliceRandom};

impl SortingAlgorithm for SelectionSort {
    fn step(&mut self, _args: &UpdateArgs) -> bool {
        // println!("Next Step: {} - {:#?}", self.looking_from, self.data);

        let mut min = self.looking_from;
        for i in (self.looking_from)..self.data.len() {
            if self.data[i] < self.data[min] {
                min = i
            }
        }

        let temp = self.data[min];
        self.data[min] = self.data[self.looking_from];
        self.data[self.looking_from] = temp;

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

pub struct SelectionSort {
    pub data: Vec<usize>,
    pub locked: bool,
    pub looking_from: usize,
}

impl SelectionSort {
    pub fn new(len: usize) -> Self {
        let mut rng = thread_rng();
        let mut items = (0usize..len).collect::<Vec<usize>>();
        items.shuffle(&mut rng);
        Self {
            data: items,
            locked: false,
            looking_from: 0,
        }
    }
}
