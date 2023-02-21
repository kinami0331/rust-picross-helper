use super::components::Line;

pub struct GameCore {
    pub row_size: u8,
    pub col_size: u8,
    pub row_setting: Vec<Vec<u8>>,
    pub col_setting: Vec<Vec<u8>>,
    pub row_lines: Vec<Line>,
    pub col_lines: Vec<Line>,
}

impl GameCore {
    pub fn new(row_size: u8, col_size: u8, row_setting: Vec<Vec<u8>>, col_setting: Vec<Vec<u8>>) -> GameCore {
        // TODO: 验证输入合法性

        let mut row_lines = Vec::<Line>::with_capacity(row_size.into());
        for i in 0..row_size {
            row_lines.push(Line::new(row_size, row_setting[i as usize].clone()));
        }

        let mut col_lines = Vec::<Line>::new();
        for i in 0..col_size {
            col_lines.push(Line::new(col_size, col_setting[i as usize].clone()));
        }

        return GameCore {
            row_size,
            col_size,
            row_setting,
            col_setting,
            row_lines,
            col_lines,
        };
    }

    pub fn get_info(&self) -> String {
        let mut rst = String::from("Game Info:\n");
        rst.push_str(&std::format!("    row size: {}\n", self.row_size));
        rst.push_str(&std::format!("    col size: {}\n", self.col_size));
        rst.push_str(&std::format!("    row_constraint: {:?}\n", self.row_setting));
        rst.push_str(&std::format!("    col_constraint: {:?}\n", self.col_setting));
        return rst;
    }
}

impl std::fmt::Debug for GameCore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Game Debug Info:")?;
        writeln!(f, "    row size: {}", self.row_size)?;
        writeln!(f, "    col size: {}", self.col_size)?;
        writeln!(f, "    row_constraint: {:?}", self.row_setting)?;
        writeln!(f, "    col_constraint: {:?}", self.col_setting)?;
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
