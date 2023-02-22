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
            row_lines.push(Line::new(row_size, row_constraint[i as usize].clone()));
        }

        let mut col_lines = Vec::<Line>::new();
        for i in 0..col_size {
            col_lines.push(Line::new(col_size, col_constraint[i as usize].clone()));
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
        writeln!(f, "    row size: {}", self.row_size)?;
        writeln!(f, "    col size: {}", self.col_size)?;
        writeln!(f, "    row_constraint: {:?}", self.row_constraint)?;
        writeln!(f, "    col_constraint: {:?}", self.col_constraint)?;
        writeln!(f, "    row_lines:")?;
        for i in 0..self.row_size {
            writeln!(f, "        {}. {:?}", i, self.row_lines[i as usize])?;
        }
        writeln!(f, "    col_lines:")?;
        for i in 0..self.col_size {
            writeln!(f, "        {}. {:?}", i, self.col_lines[i as usize])?;
        }
        write!(f, "")
    }
}
