#[derive(Debug)]
pub struct GameCore {
    pub row_size: u32,
    pub col_size: u32,
}

impl GameCore {
    pub fn new(row_size: u32, col_size: u32) -> GameCore {
        return GameCore { row_size, col_size };
    }

    pub fn get_info(&self) -> String {
        let mut rst = String::from("Game Info:\n");
        rst.push_str(&std::format!("    row size: {}\n", self.row_size));
        rst.push_str(&std::format!("    col size: {}\n", self.col_size));
        return rst;
    }
}
