use super::chess_types::*;


#[derive(Debug, PartialEq, Eq)]
pub struct MoveError;

#[derive(Debug, PartialEq, Eq)]
pub struct AttackError;

pub type MovesResult = Result<Vec<Address>, MoveError>;

pub fn get_piece_moves(board: &Board, address: Address) -> MovesResult {
    let piece = board.get_cell(address)
        .as_ref()
        .ok_or(MoveError)?;
    
    let mut res = Vec::<Address>::new();

    let f = match piece.piece_type {
        PieceType::Pawn   => get_pawn_moves,
        PieceType::Knight => get_knight_moves,
        PieceType::Bishop => get_bishop_moves,
        PieceType::Rook   => get_rook_moves,
        PieceType::Queen  => get_queen_moves,
        PieceType::King   => get_king_moves,
    };
    f(board, address, piece.color, &mut res);
    Ok(res)
}

fn get_pawn_moves(board: &Board, address: Address, color: Color, out: &mut Vec<Address>) {
    if color == Color::White {
        address.get_shifted(1, 0)
    }
}

fn get_knight_moves(board: &Board, address: Address, _color: Color, out: &mut Vec<Address>) {

}

fn get_bishop_moves(board: &Board, address: Address, _color: Color, out: &mut Vec<Address>) {

}

fn get_rook_moves(board: &Board, address: Address, _color: Color, out: &mut Vec<Address>) {

}

fn get_queen_moves(board: &Board, address: Address, _color: Color, out: &mut Vec<Address>) {

}

fn get_king_moves(board: &Board, address: Address, _color: Color, out: &mut Vec<Address>) {

}