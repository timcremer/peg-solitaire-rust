use omas_spiel::board::Board;

#[test]
fn initial_pegs() {
    let board = Board::new();
    assert_eq!(board.pegs_remaining(), 32);
}

#[test]
fn move_success() {
    let mut board = Board::new();
    assert!(board.move_peg(3, 1, 0, 2));
    assert_eq!(board.pegs_remaining(), 31);
}

#[test]
fn move_failure() {
    let mut board = Board::new();
    assert!(!board.move_peg(0, 0, 0, 2));
}
