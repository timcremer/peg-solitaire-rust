mod board;
mod gui;
mod solver;
use gui::OmasApp;

fn main() {
    let mut board = board::Board::new();
    let mut solution = solver::Solution::new();
    //solution.solve(&mut board);
    //solution.print_moves();
    
    let options = eframe::NativeOptions::default();
    
    eframe::run_native(
        "Omas Spiel",
        options,
        Box::new(|_cc| Box::new(OmasApp::default())),
    );
}
