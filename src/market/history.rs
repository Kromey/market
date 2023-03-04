use std::{ops::Add, iter::Sum};


#[derive(Debug, PartialEq, Eq)]
pub struct History<const N: usize> {
    history: [Statistics; N],
    current_idx: usize,
}

impl<const N: usize> History<N> {
    pub fn new() -> Self {
        Self {
            history: [Statistics::default(); N],
            current_idx: 0,
        }
    }

    pub fn get_latest(&self) -> Statistics {
        self.history[self.current_idx]
    }

    pub fn get_totals(&self) -> Statistics {
        self.history.iter().sum()
    }

    pub fn push(&mut self, entry: Statistics) {
        self.current_idx = (self.current_idx + 1) % N;
        self.history[self.current_idx] = entry;
    }

    pub fn insert(&mut self, volume: u32, value: u32) {
        let stats = Statistics {
            volume,
            value,
        };
        self.push(stats);
    }
}

impl<const N: usize> Default for History<N> {
    fn default() -> Self {
        History::new()
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Statistics {
    volume: u32,
    value: u32,
}

impl Statistics {
    pub fn volume(&self) -> u32 {
        self.volume
    }

    pub fn value(&self) -> u32 {
        self.value
    }

    pub fn price(&self) -> u32 {
        (self.value as f32 / self.volume as f32).round() as u32
    }
}

impl Add<Statistics> for Statistics {
    type Output = Statistics;

    fn add(self, rhs: Statistics) -> Self::Output {
        Statistics {
            volume: self.volume + rhs.volume,
            value: self.value + rhs.value,
        }
    }
}

impl Sum for Statistics {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Statistics::default(), |acc, stats| acc + stats)
    }
}

impl<'a> Sum<&'a Statistics> for Statistics {
    fn sum<I: Iterator<Item = &'a Statistics>>(iter: I) -> Self {
        iter.fold(Statistics::default(), |acc, stats| acc + *stats)
    }
}
