mod core;
use crate::core::chess_types::*;

fn main() {
    let mut board = Board::new();
    //board.flip_for(Color::Black);
    println!("{}", board);
    board.flip();
    println!("{}", board);
}
