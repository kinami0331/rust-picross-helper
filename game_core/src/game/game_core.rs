use super::components::Line;
use serde::{Deserialize, Serialize};

pub struct GameCore {
    pub row_size: u8,
    pub col_size: u8,
    pub row_constraint: Vec<Vec<u8>>,
    pub col_constraint: Vec<Vec<u8>>,
    pub row_lines: Vec<Line>,
    pub col_lines: Vec<Line>,
}

#[derive(Serialize, Deserialize)]
pub struct GameInfo {
    pub row_size: u8,
    pub col_size: u8,
    pub row_constraint: Vec<Vec<u8>>,
    pub col_constraint: Vec<Vec<u8>>,
}

impl GameCore {
    pub fn new(row_size: u8, col_size: u8, row_constraint: Vec<Vec<u8>>, col_constraint: Vec<Vec<u8>>) -> GameCore {
        // TODO: 验证输入合法性

        let mut row_lines = Vec::<Line>::with_capacity(row_size.into());
        for i in 0..row_size {
            row_lines.push(Line::new(col_size, row_constraint[i as usize].clone()));
        }

        let mut col_lines = Vec::<Line>::new();
        for i in 0..col_size {
            col_lines.push(Line::new(row_size, col_constraint[i as usize].clone()));
        }

        GameCore {
            row_size,
            col_size,
            row_constraint,
            col_constraint,
            row_lines,
            col_lines,
        }
    }

    pub fn from_json(json_str: &str) -> GameCore {
        let game_info: GameInfo = match serde_json::from_str(json_str) {
            Ok(game_info) => game_info,
            Err(_) => panic!("json读取失败"),
        };

        GameCore::new(
            game_info.row_size,
            game_info.col_size,
            game_info.row_constraint,
            game_info.col_constraint,
        )
    }
}

impl GameCore {
    pub fn set_cell(&mut self, row: u8, col: u8, value: u8) -> &mut GameCore {
        if row >= self.row_size || col >= self.col_size {
            return self;
        }
        if self.row_lines[row as usize].confirmed_at(col) {
            return self;
        }
        self.row_lines[row as usize].set_at(col, value);
        self.col_lines[col as usize].set_at(row, value);
        self
    }
}

// solver 相关
impl GameCore {
    pub fn solve(&mut self) {
        loop {
            let mut changed = false;
            // 遍历行
            for row_idx in 0..self.row_size {
                if self.row_lines[row_idx as usize].confirmed_mask == 0 {
                    continue;
                }
                changed = true;

                let line = &mut self.row_lines[row_idx as usize];
                for col_idx in 0..self.col_size {
                    if (line.confirmed_mask >> col_idx) & 1 == 0 {
                        continue;
                    }
                    if (line.confirmed_true_mask >> col_idx) & 1 != 0 {
                        self.col_lines[col_idx as usize].set_at(row_idx, 1);
                    } else {
                        self.col_lines[col_idx as usize].set_at(row_idx, 0);
                    }
                }
                line.apply_confirmed_mask();
            }

            // 遍历列
            for col_idx in 0..self.col_size {
                if self.col_lines[col_idx as usize].confirmed_mask == 0 {
                    continue;
                }
                changed = true;

                let line = &mut self.col_lines[col_idx as usize];
                for row_idx in 0..self.row_size {
                    if (line.confirmed_mask >> row_idx) & 1 == 0 {
                        continue;
                    }
                    if (line.confirmed_true_mask >> row_idx) & 1 != 0 {
                        self.row_lines[row_idx as usize].set_at(col_idx, 1);
                    } else {
                        self.row_lines[row_idx as usize].set_at(col_idx, 0);
                    }
                }
                line.apply_confirmed_mask();
            }

            // debug assert
            // for i in 0..self.row_size {
            //     for j in 0..self.col_size {
            //         assert_eq!(
            //             self.row_lines[i as usize].confirmed_at(j),
            //             self.col_lines[j as usize].confirmed_at(i)
            //         );
            //     }
            // }
            // ---------

            if !changed {
                break;
            }
        }
    }
}

impl GameCore {
    pub fn get_info(&self) -> String {
        let mut rst = String::from("Game Info:\n");
        rst.push_str(&std::format!("    row size: {}\n", self.row_size));
        rst.push_str(&std::format!("    col size: {}\n", self.col_size));
        rst.push_str(&std::format!("    row_constraint: {:?}\n", self.row_constraint));
        rst.push_str(&std::format!("    col_constraint: {:?}\n", self.col_constraint));
        return rst;
    }
}

impl std::fmt::Debug for GameCore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Game Debug Info:")?;
        writeln!(f, "  row size: {}", self.row_size)?;
        writeln!(f, "  col size: {}", self.col_size)?;
        writeln!(f, "  row_constraint: {:?}", self.row_constraint)?;
        writeln!(f, "  col_constraint: {:?}", self.col_constraint)?;
        writeln!(f, "  row_lines:")?;
        for i in 0..self.row_size {
            writeln!(f, "    row #{}:", i)?;
            write!(f, "{}", self.row_lines[i as usize].debug_info(6, f.alternate()))?;
        }
        writeln!(f, "  col_lines:")?;
        for i in 0..self.row_size {
            writeln!(f, "    col #{}:", i)?;
            write!(f, "{}", self.col_lines[i as usize].debug_info(6, f.alternate()))?;
        }
        write!(f, "")
    }
}

impl std::fmt::Display for GameCore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.sign_minus() {
            for i in 0..self.row_size {
                writeln!(f, "{}", self.row_lines[i as usize].debug_line_status_str())?;
            }
            writeln!(f, "")
        } else {
            writeln!(f, "{:?}", self)
        }
    }
}
