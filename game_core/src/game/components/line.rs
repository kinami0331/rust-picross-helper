use crate::utils;
use std::collections::HashSet;

pub struct Line {
    pub size: u8,
    pub confirmed: u64,
    pub filled: u64,
    pub constraint: Vec<u8>,

    pub valid_number: u64,
    pub valid_set: HashSet<u64>,
}

impl Line {
    pub fn new(size: u8, constraint: Vec<u8>) -> Line {
        let valid_number = Line::calc_init_valid_number(size, &constraint);
        return Line {
            size,
            confirmed: 0,
            filled: 0,
            constraint,
            valid_number,
            valid_set: HashSet::new(),
        };
    }

    fn calc_init_valid_number(size: u8, constraint: &Vec<u8>) -> u64 {
        if constraint.len() == 1 && constraint[0] == 0 {
            1
        } else {
            let fill_num = constraint.iter().fold(0, |acc, v| acc + v);
            let blank_num = size - fill_num;
            let seg_num = constraint.len();
            utils::count_combinations(blank_num as u64 + 1, seg_num as u64)
        }
    }
}

impl Line {
    pub fn confirmed_at(&self, index: u8) -> bool {
        return (self.confirmed >> index) & 1 == 1;
    }

    pub fn set_at(&mut self, index: u8, value: u8) {
        self.confirmed |= 1 << index;
        if value != 0 {
            self.filled |= 1 << index;
        }
    }
}

impl std::fmt::Debug for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "| ")?;
        for i in 0..self.size {
            if i > 0 {
                write!(f, " ")?;
            }
            if (self.confirmed >> i) & 1 != 0 {
                if (self.filled >> i) & 1 != 0 {
                    write!(f, "o")?;
                } else {
                    write!(f, "x")?;
                }
            } else {
                write!(f, "-")?;
            }
        }
        write!(f, " | current valid number: {}", self.valid_number)
    }
}
