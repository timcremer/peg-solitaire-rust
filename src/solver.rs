use crate::board::Board;

pub struct Solution{
    moves: Vec<((usize, usize), (usize, usize))>,
}

impl Solution {
    pub fn new() -> Self {
        Solution {
            moves: Vec::new(),
        }
    }


    pub fn solve(&mut self, board: &mut Board) -> bool {
        //println!("Trying to solve board:");
        if board.classic_win() {
            println!("Solution found with {} moves!", self.moves.len());
            return true;
        }
        for r in 0..6 {
            for d in 0..6 {
                
                if board.possible_to_move_peg(r, d) {
                    for (dr, dd) in Board::MOVES.iter() {
                        let r_to = r as isize + dr;
                        let d_to = d as isize + dd;
                        
                       if board.possible_to_move_peg_to(r, d, r_to as usize, d_to as usize) {
                            //println!("Moving peg from ({}, {}) to ({}, {})", r, d, r_to, d_to);
                            board.move_peg(r, d, r_to as usize, d_to as usize);
                            self.moves.push(((r, d), (r_to as usize, d_to as usize)));
                            if self.solve(board) {
                                return true;
                            }
                            board.undo_move(r, d, r_to as usize, d_to as usize);
                            self.moves.pop();
                            //println!("{:?}", board.pegs_remaining());
                        }
                    }
                }
                
            }
        }
        false

    }

    pub fn print_moves(&self) {
        for (from, to) in &self.moves {
            println!("Move peg from ({}, {}) to ({}, {})", from.0, from.1, to.0, to.1);
        }
    }
}