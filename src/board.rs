pub struct Board {
    size: usize,
    grid: Vec<Vec<Option<bool>>>,
}

impl Board {
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn cell(&self, r: usize, c: usize) -> Option<bool> {
        self.grid[r][c]
    }

    pub fn new() -> Self {
        let size = 7;
        let mut grid = vec![vec![None; size]; size];
        for r in 0..size {
            for c in 0..size {
                if (r < 2 || r > 4) && (c < 2 || c > 4) {
                    grid[r][c] = None;
                } else {
                    grid[r][c] = Some(true);
                }
            }
        }
        grid[3][3] = Some(false);
        Board { size, grid }
    }

    pub fn display(&self) {
        for row in &self.grid{
            let row_str: String = row.iter().map(|cell| {
                match cell {
                    Some(true) => "● ",
                    Some(false) => "○ ",
                    None => "  ",
                }
            }).collect();
            println!("{}", row_str);
        }
    }

    pub fn possible_to_move_peg(&mut self, r: usize, d: usize, ) -> bool {
        if r >= self.size || d >= self.size || self.grid[r][d].is_none() || self.grid[r][d] == Some(false) {
            return false;
        }
        
        if r >= 2 && self.grid[r-1][d] == Some(true) && self.grid[r-2][d] == Some(false) {
            return true;
        }
        if r <= self.size - 3 && self.grid[r+1][d] == Some(true) && self.grid[r+2][d] == Some(false) {
            return true;
        }
        if d >= 2 && self.grid[r][d-1] == Some(true) && self.grid[r][d-2] == Some(false) {
            return true;
        }
        if d <= self.size - 3 && self.grid[r][d+1] == Some(true) && self.grid[r][d+2] == Some(false) {
            return true;
        }
        false
    }

    pub fn possible_to_move_peg_to(&mut self, r_peg: usize, d_peg: usize,r_to: usize, d_to: usize) -> bool {
        if r_peg >= self.size || d_peg >= self.size || self.grid[r_peg][d_peg].is_none() || self.grid[r_peg][d_peg] == Some(false) {
            return false;
        }
        if r_to >= self.size || d_to >= self.size || self.grid[r_to][d_to].is_none() || self.grid[r_to][d_to] == Some(true) {
            return false;
        }

        if r_to == r_peg && d_to == d_peg + 2 && self.grid[r_peg][d_peg + 1] == Some(true) {
            return true;
        }
        if r_to == r_peg && d_to + 2 == d_peg && self.grid[r_peg][d_to + 1] == Some(true) {
            return true;
        }
        if d_to == d_peg && r_to == r_peg + 2 && self.grid[r_peg + 1][d_peg] == Some(true) {
            return true;
        }
        if d_to == d_peg && r_to + 2 == r_peg && self.grid[r_to + 1][d_peg] == Some(true) {
            return true;
        }
        false
    }

    pub fn move_peg(&mut self, r_peg: usize, d_peg: usize,r_to: usize, d_to: usize) -> bool {
        if self.possible_to_move_peg_to(r_peg, d_peg, r_to, d_to) {
            self.grid[r_peg][d_peg] = Some(false);
            self.grid[r_to][d_to] = Some(true);
            if r_to == r_peg {
                self.grid[r_peg][(d_peg + d_to) / 2] = Some(false);
            } else {
                self.grid[(r_peg + r_to) / 2][d_peg] = Some(false);
            }
            return true;
        }
        false
    }

    pub fn pegs_remaining(&self) -> usize {
        self.grid.iter().flatten().filter(|&&cell| cell == Some(true)).count()
    }

    pub fn classic_win(&self) -> bool {
        self.pegs_remaining() == 1 && self.grid[3][3] == Some(true)
    }


}
