mod board;

use board::*;

fn main() {
    let mut board = Board::default();
    board.set(1, 1, BCell::X);
    board.set(0, 0, BCell::O);
    println!("{}", board)
}
