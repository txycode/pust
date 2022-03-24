#[derive(Debug, Clone)]
pub struct PosCtx {
    pub pos_start: Box<Position>,
    pub pos_end: Box<Position>,
}

#[derive(Clone, Debug)]
pub struct Position {
    pub idx: usize,
    pub col: u128,
    pub row: u128,
    pub fd: String,
    pub text: String,
    pub len: usize,
}

impl Position {
    pub fn advance(&mut self, cur_char: char) {
        self.idx += 1;
        self.col += 1;
        if cur_char == '\n' {
            self.col = 0;
            self.row += 1;
        }
    }
}

impl PosCtx {
    pub fn merge(&self, other: &Box<PosCtx>) -> Box<PosCtx> {
        Box::new(PosCtx {
            pos_start: self.pos_start.clone(),
            pos_end: other.pos_end.clone(),
        })
    }
}
