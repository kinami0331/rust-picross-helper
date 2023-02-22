use crate::utils;
use std::collections::HashSet;

pub struct Line {
    pub size: u8,
    pub confirmed: u64,
    pub filled: u64,
    pub constraint: Vec<u8>,

    pub valid_number: u64,
    pub valid_set: HashSet<u64>,
    pub comfirmed_mask: u64,
}

impl Line {
    pub fn new(size: u8, constraint: Vec<u8>) -> Line {
        let valid_number = Line::calc_init_valid_number(size, &constraint);
        let valid_set = Line::gen_init_valid_set(size, &constraint);
        let true_and = valid_set.iter().fold(!0, |acc, e| acc & e);
        let false_and = valid_set.iter().fold(!0, |acc, e| acc & (!e));
        let comfirmed_mask = true_and | false_and;

        return Line {
            size,
            confirmed: 0,
            filled: 0,
            constraint,
            valid_number,
            valid_set,
            comfirmed_mask,
        };
    }

    pub fn calc_init_valid_number(size: u8, constraint: &Vec<u8>) -> u64 {
        if constraint.len() == 1 && constraint[0] == 0 {
            1
        } else {
            let fill_num = constraint.iter().fold(0, |acc, v| acc + v);
            let blank_num = size - fill_num;
            let seg_num = constraint.len();
            utils::count_combinations(blank_num as u64 + 1, seg_num as u64)
        }
    }

    pub fn gen_init_valid_set(size: u8, constraint: &Vec<u8>) -> HashSet<u64> {
        let mut rst = HashSet::new();
        if constraint.len() == 1 && constraint[0] == 0 {
            rst.insert(0);
            return rst;
        }

        let fill_num = constraint.iter().fold(0, |acc, v| acc + v);
        let blank_num = size - fill_num;
        let seg_num = constraint.len() as u8;

        let seg_bits: Vec<u64> = constraint.iter().map(|x| utils::gen_bits_by_num(*x)).collect();
        let intervals = utils::calc_balls_in_boxes_combination(blank_num + 2, seg_num + 1);

        for interval in intervals {
            let mut ans: u64 = 0;
            let mut cur_idx = 0;
            for idx in 0..(interval.len() - 1) {
                cur_idx += interval[idx];
                if idx == 0 {
                    cur_idx -= 1;
                }
                ans |= seg_bits[idx] << cur_idx;
                cur_idx += constraint[idx];
            }

            rst.insert(ans);
        }

        rst
    }
}

impl Line {
    pub fn confirmed_at(&self, index: u8) -> bool {
        return (self.confirmed >> index) & 1 == 1;
    }

    pub fn set_at(&mut self, index: u8, value: u8) {
        // TODO: 后面改成更健壮的检查
        assert!(!self.confirmed_at(index));
        self.confirmed |= 1 << index;
        if value != 0 {
            self.filled |= 1 << index;
        }
        // 更新 valid set
        self.valid_set.retain(|x| (x >> index) & 1 == value as u64);
        self.valid_number = self.valid_set.len() as u64;
        assert_ne!(self.valid_number, 0);
        let true_and = self.valid_set.iter().fold(!0, |acc, e| acc & e);
        let false_and = self.valid_set.iter().fold(!0, |acc, e| acc & (!e));
        self.comfirmed_mask = true_and | false_and;
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

impl Line {
    pub fn line_str_from_bits(size: u8, bits: u64) -> String {
        let mut rst = String::new();
        for i in 0..size {
            if i > 0 {
                rst += " ";
            }
            if (bits >> i) & 1 != 0 {
                rst += "o";
            } else {
                rst += "x";
            }
        }
        rst
    }

    pub fn debug_line_status_str(&self) -> String {
        let mut rst = String::new();
        for i in 0..self.size {
            if i > 0 {
                rst += " ";
            }
            if (self.confirmed >> i) & 1 != 0 {
                if (self.filled >> i) & 1 != 0 {
                    rst += "o";
                } else {
                    rst += "x";
                }
            } else {
                rst += "-";
            }
        }
        rst
    }

    pub fn debug_comfirmed_mask_status(&self) -> String {
        let mut rst = String::new();
        for i in 0..self.size {
            if i > 0 {
                rst += " ";
            }
            // 已经放置了具体值的直接显示 "-"
            if (self.confirmed >> i) & 1 == 0 {
                // 如果这一位已经能确定，显示对应的符号
                if (self.comfirmed_mask >> i) & 1 != 0 {
                    let t = *self.valid_set.iter().next().unwrap();
                    if (t >> i) & 1 != 0 {
                        rst += "o";
                    } else {
                        rst += "x";
                    }
                } else {
                    rst += "?"
                }
            } else {
                rst += "-";
            }
        }
        rst
    }

    pub fn debug_info(&self, base_indent: u32, detailed: bool) -> String {
        let mut rst = String::new();
        let mut indent = String::new();
        for _ in 0..base_indent {
            indent += " ";
        }
        rst += &format!("{}line constraint: {:?}\n", indent, self.constraint);
        rst += &format!(
            "{}cell status: \n        | {} |\n",
            indent,
            self.debug_line_status_str()
        );
        rst += &format!("{}current valid number: {}\n", indent, self.valid_number);
        rst += &format!(
            "{}current comfirmed from valid:\n        | {} |\n",
            indent,
            self.debug_comfirmed_mask_status()
        );
        if detailed {
            rst += &format!("{}current valid list:\n", indent);
            for bits in &self.valid_set {
                rst += &format!("{}  | {} |\n", indent, Line::line_str_from_bits(self.size, *bits));
            }
        }

        rst
    }
}
