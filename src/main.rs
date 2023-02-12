mod core;
use crate::core::chess_types::*;

fn main() {
    let board = Board::new();
    println!("{}", board);
}
