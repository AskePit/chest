mod core;
use crate::core::chess_types::*;
use crate::core::*;
use web_view::*;

fn main() {
    let mut board = Board::new();
    //game_engine::make_move(&mut board, Address::parse("e2"), Address::parse("e4"));

    let _ = game_engine::make_moves(&mut board, vec![
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
    ]);

    println!("{}", board);

    let res
        = game_engine::get_piece_moves(&board, Address::parse("e1"));
    assert_eq!(res.is_ok(), true);

    for addr in res.unwrap() {
        print!("{} ", addr);
    }

    let html_content = format!("<html><body><h1>{}</h1></body></html>", board).replace("\n", "<br/>");
	
    web_view::builder()
        .title("Chest")
        .content(Content::Html(include_str!("gui/index.html")))
        .size(614, 614)
        .resizable(false)
        .debug(false)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}
