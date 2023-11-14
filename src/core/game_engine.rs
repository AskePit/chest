use std::str::FromStr;

use super::chess_types::*;

// MOVE OFFSETS
// tuple (x, y) where
// x is column: negative left, positive right
// y is row: negative back, positive forwards
//
// offsets are given for white pieces
// for black pieces offsets should be rotated by 180 degrees
static PAWN_MARCH_OFFSET: &[(i8, i8)] = &[(0, 1)];

static PAWN_LONG_MARCH_OFFSET: &[(i8, i8)] = &[(0, 2)];

static PAWN_CAPTURE_OFFSETS: &[(i8, i8)] = &[(-1, 1), (1, 1)];

static KNIGHT_MOVE_OFFSETS: &[(i8, i8)] = &[
    (-1, 2),
    (1, 2),
    (2, 1),
    (2, -1),
    (1, -2),
    (-1, -2),
    (-2, -1),
    (-2, 1),
];

static BISHOP_MOVE_OFFSETS: &[(i8, i8)] = &[(-1, -1), (-1, 1), (1, -1), (1, 1)];

static ROOK_MOVE_OFFSETS: &[(i8, i8)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

static KING_QUEEN_MOVE_OFFSETS: &[(i8, i8)] = &[
    (-1, 0),
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
];

#[derive(Debug, PartialEq, Eq)]
pub enum MoveError {
    InvalidAddress(ParseAddressError),
    NoPiece,
    WrongColorTurn(Color),
    UnreachableMove { from: Address, to: Address },
}

pub type MovesResult = Result<Vec<Address>, MoveError>;

pub fn get_piece_moves(board: &Board, address: Address) -> MovesResult {
    let piece = board.get_cell(address).as_ref().ok_or(MoveError::NoPiece)?;

    let mut res = Vec::<Address>::new();

    let f = match piece.piece_type {
        PieceType::Pawn => get_pawn_moves,
        PieceType::Knight => get_knight_moves,
        PieceType::Bishop => get_bishop_moves,
        PieceType::Rook => get_rook_moves,
        PieceType::Queen => get_queen_moves,
        PieceType::King => get_king_moves,
    };
    f(board, address, piece.color, &mut res);
    Ok(res)
}

fn get_pawn_moves(board: &Board, address: Address, color: Color, out: &mut Vec<Address>) {
    static WHITE_PAWN_INITIAL_ROW: u8 = 1; // like e2
    static BLACK_PAWN_INITIAL_ROW: u8 = 6; // like e7
    let is_initial_row = (color == Color::White && address.row == WHITE_PAWN_INITIAL_ROW)
        || (color == Color::Black && address.row == BLACK_PAWN_INITIAL_ROW);

    let rotate_by_color = |offset: (i8, i8)| -> (i8, i8) {
        if color == Color::White {
            offset
        } else {
            (-offset.0, -offset.1)
        }
    };

    // long march
    if is_initial_row {
        let long_march = rotate_by_color(PAWN_LONG_MARCH_OFFSET[0]);
        if let Some(move_address) = address.get_shifted(long_march) {
            if let None = *board.get_cell(move_address) {
                out.push(move_address);
            }
        }
    }

    // usual march
    let normal_march = rotate_by_color(PAWN_MARCH_OFFSET[0]);
    if let Some(move_address) = address.get_shifted(normal_march) {
        if let None = *board.get_cell(move_address) {
            out.push(move_address);
        }
    }

    // captures
    for capture_offset in PAWN_CAPTURE_OFFSETS {
        if let Some(move_address) = address.get_shifted(rotate_by_color(*capture_offset)) {
            if let Some(ref piece) = *board.get_cell(move_address) {
                if piece.color != color {
                    out.push(move_address);
                }
            }
        }
    }
}

fn get_scalar_piece_moves(
    scalar_offsets: &[(i8, i8)],
    board: &Board,
    address: Address,
    color: Color,
    out: &mut Vec<Address>,
) {
    for offset in scalar_offsets {
        if let Some(move_address) = address.get_shifted(*offset) {
            if let Some(ref piece) = *board.get_cell(move_address) {
                if piece.color != color {
                    out.push(move_address);
                }
            } else {
                out.push(move_address);
            }
        }
    }
}

fn get_vector_piece_moves(
    vector_offsets: &[(i8, i8)],
    board: &Board,
    address: Address,
    color: Color,
    out: &mut Vec<Address>,
) {
    for offset in vector_offsets {
        let mut addr = address.get_shifted(*offset);
        while let Some(move_address) = addr {
            if let Some(ref piece) = *board.get_cell(move_address) {
                if piece.color != color {
                    out.push(move_address);
                }
                break;
            } else {
                out.push(move_address);
            }

            addr = move_address.get_shifted(*offset);
        }
    }
}

fn get_knight_moves(board: &Board, address: Address, color: Color, out: &mut Vec<Address>) {
    get_scalar_piece_moves(KNIGHT_MOVE_OFFSETS, board, address, color, out);
}

fn get_bishop_moves(board: &Board, address: Address, color: Color, out: &mut Vec<Address>) {
    get_vector_piece_moves(BISHOP_MOVE_OFFSETS, board, address, color, out);
}

fn get_rook_moves(board: &Board, address: Address, color: Color, out: &mut Vec<Address>) {
    get_vector_piece_moves(ROOK_MOVE_OFFSETS, board, address, color, out);
}

fn get_queen_moves(board: &Board, address: Address, color: Color, out: &mut Vec<Address>) {
    get_vector_piece_moves(KING_QUEEN_MOVE_OFFSETS, board, address, color, out);
}

fn get_king_moves(board: &Board, address: Address, color: Color, out: &mut Vec<Address>) {
    get_scalar_piece_moves(KING_QUEEN_MOVE_OFFSETS, board, address, color, out);
}

pub fn make_move(board: &mut Board, from: Address, to: Address) -> Result<(), MoveError> {
    if let Some(piece) = board.get_cell(from) {
        if piece.color != board.whose_turn {
            return Err(MoveError::NoPiece);
        }
    }
    let possible_moves = get_piece_moves(&board, from)?;

    if possible_moves.contains(&to) {
        board.move_piece(from, to);
        board.flip_player();
        Ok(())
    } else {
        Err(MoveError::UnreachableMove { from, to })
    }
}

pub fn make_moves(board: &mut Board, moves: Vec<(&str, &str)>) -> Result<(), MoveError> {
    for m in moves {
        let wrap_error = |parse_error| MoveError::InvalidAddress(parse_error);

        make_move(
            board,
            Address::from_str(m.0).map_err(wrap_error)?,
            Address::from_str(m.1).map_err(wrap_error)?,
        )?
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn board_moves() {
        let board = Board::new();
        let res = get_piece_moves(&board, Address::from_str("e2").unwrap());
        assert_eq!(res.is_ok(), true);

        println!("{:?}", res.unwrap());
    }
}
