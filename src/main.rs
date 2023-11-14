mod core;

use crate::core::chess_types::*;
use crate::core::*;
use std::ops::Deref;
use web_view::*;

fn main() {
    let mut board = Board::new();

    let _ = game_engine::make_moves(
        &mut board,
        vec![
            ("e2", "e4"),
            ("c7", "c6"),
            ("b1", "c3"),
            ("d7", "d5"),
            ("e4", "d5"),
            ("d5", "c6"),
            ("c6", "c7"),
            ("c7", "d8"),
            ("e8", "d8"),
            ("c8", "h3"),
            ("g2", "h3"),
            ("h3", "g4"),
        ],
    );

    println!("{}", board);

    let res = game_engine::get_piece_moves(&board, Address::parse("e1"));
    assert_eq!(res.is_ok(), true);

    for addr in res.unwrap() {
        println!("{} ", addr);
    }

    let mut webview = web_view::builder()
        .title("Chest")
        .content(Content::Html(include_str!("gui/index.html")))
        .size(1000, 900)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|webview, arg| {
            match arg {
                "syncRequest" => {
                    let objects = board
                        .pieces
                        .iter()
                        .enumerate()
                        .filter(|x| x.1.is_some())
                        .map(|(i, x)| (i, x.unwrap()))
                        .map(|(i, x)| {
                            format!(
                                "{{address: {}, piece_type: {}, color: {}}}",
                                i, x.piece_type as u8, x.color as u8
                            )
                        })
                        .collect::<Vec<_>>()
                        .join(",");

                    let objects = format!("sync([{}])", objects);

                    println!("{}", objects);

                    webview.eval(&objects)
                }
                "test_two" => {
                    println!("cringe2!");
                    // Invoke a JavaScript function!
                    webview.eval(&format!("myFunction({}, {})", 123, 456))
                }
                _ => Ok(()),
            }
        })
        .build()
        .unwrap();

    webview.set_maximized(true);
    webview.run().unwrap();
}
