pub struct Line {
    pub size: u8,
    pub confirmed: u64,
    pub filled: u64,
    pub constraint: Vec<u8>,
}

impl Line {
    pub fn new(size: u8, constraint: Vec<u8>) -> Line {
        return Line {
            size,
            confirmed: 0,
            filled: 0,
            constraint,
        };
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
        write!(f, "")
    }
}
