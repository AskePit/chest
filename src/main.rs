mod core;
use crate::core::chess_types::*;
use crate::core::*;
use std::str::FromStr;

fn main() {
    let mut board = Board::new();
    //board.flip_for(Color::Black);
    println!("{}", board);
    board.flip();
    println!("{}", board);

    let res = game_engine::get_piece_moves(&board, Address::from_str("g1").unwrap());
    assert_eq!(res.is_ok(), true);

    for addr in res.unwrap() {
        print!("{} ", addr);
    }
}
