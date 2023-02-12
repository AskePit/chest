use std::{fmt::Display, str::FromStr};

#[derive(Debug, Copy, Clone)]
pub enum Color {
    White,
    Black
}

#[derive(Debug, Copy, Clone)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

impl PieceType {
    pub fn get_value(&self, phase: GamePhase) -> u32 {
        let value = match self {
            PieceType::Pawn   => 1,
            PieceType::Knight => 3,
            PieceType::Bishop => 3,
            PieceType::Rook   => 5,
            PieceType::Queen  => 9,
            PieceType::King   => u32::MAX,
        };

        match phase {
            GamePhase::Opening    => value,
            GamePhase::Middlegame => value,
            GamePhase::Endgame    => value,
        }
    }
}

#[derive(Debug)]
pub enum GamePhase {
    Opening,
    Middlegame,
    Endgame
}

#[derive(Debug)]
pub enum Side {
    King,
    Queen,
}

#[derive(Debug, Copy, Clone)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match self.color {
            Color::White => {
                match self.piece_type {
                    PieceType::Pawn   => "♙",
                    PieceType::Knight => "♘",
                    PieceType::Bishop => "♗",
                    PieceType::Rook   => "♖",
                    PieceType::Queen  => "♕",
                    PieceType::King   => "♔",
                }
            },
            Color::Black => {
                match self.piece_type {
                    PieceType::Pawn   => "♟︎",
                    PieceType::Knight => "♞",
                    PieceType::Bishop => "♝",
                    PieceType::Rook   => "♜",
                    PieceType::Queen  => "♛",
                    PieceType::King   => "♚",
                }
            },
        };
        write!(f, "{}", val)
    }
}

pub const ROW_SIZE: u8 = 8u8;
pub const CELLS_COUNT: u8 = ROW_SIZE * ROW_SIZE;
pub type BoardLayer<T> = [T; CELLS_COUNT as usize];

pub struct Address {
    pub row: u8,
    pub col: u8
}

impl Address {
    pub fn new(row: u8, col: u8) -> Self {
        Address { row, col }
    }

    pub fn get_row_name(row: u8) -> char {
        ('1' as u8 + row) as char
    }

    pub fn get_col_name(col: u8) -> char {
        ('a' as u8 + col) as char
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseAddressError;

impl FromStr for Address {
    type Err = ParseAddressError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut res: Self = Address{row: 0, col:0};

        let chars = s.chars().collect::<Vec<_>>();

        if chars.len() != 2 {
            return Err(ParseAddressError)
        }

        let col = chars[0].to_ascii_lowercase();
        let row = chars[1].to_ascii_lowercase();

        if let c @ 'a'..='h' = col {
            res.col = (c as u8) - ('a' as u8)
        } else {
            return Err(ParseAddressError)
        }

        res.row = row.to_digit(10).ok_or(ParseAddressError)? as u8;

        if res.row < 1 || res.row > 8 {
            return Err(ParseAddressError)
        }
        
        Ok(res)
    }
}

pub struct Board {
    pub pieces: BoardLayer<Option<Piece>>,
}

impl Board {
    pub fn new() -> Self {
        let spawn_piece = |color: Color, piece_type: PieceType| -> Option<Piece> {
            Some(Piece {color, piece_type})
        };

        let w = |t: PieceType| -> Option<Piece> {
            spawn_piece(Color::White, t)
        };

        let b = |t: PieceType| -> Option<Piece> {
            spawn_piece(Color::Black, t)
        };

        use PieceType::*;
        Board {
            pieces: [
                w(Rook), w(Knight), w(Bishop), w(Queen), w(King), w(Bishop), w(Knight), w(Rook),
                w(Pawn), w(Pawn),   w(Pawn),   w(Pawn),  w(Pawn), w(Pawn),   w(Pawn),   w(Pawn),
                None,    None,      None,      None,     None,    None,      None,      None,
                None,    None,      None,      None,     None,    None,      None,      None,
                None,    None,      None,      None,     None,    None,      None,      None,
                None,    None,      None,      None,     None,    None,      None,      None,
                b(Pawn), b(Pawn),   b(Pawn),   b(Pawn),  b(Pawn), b(Pawn),   b(Pawn),   b(Pawn),
                b(Rook), b(Knight), b(Bishop), b(Queen), b(King), b(Bishop), b(Knight), b(Rook)
            ]
        }
    }

    pub fn new_empty() -> Self {
        Board {
            pieces: [None; CELLS_COUNT as usize]
        }
    }

    fn get_index(address: Address) -> u8 {
        address.row * ROW_SIZE + address.col
    }

    pub fn get_cell(&self, address: Address) -> &Option<Piece> {
        &self.pieces[Self::get_index(address) as usize]
    }

    pub fn get_cell_mut(&mut self, address: Address) -> &mut Option<Piece> {
        &mut self.pieces[Self::get_index(address) as usize]
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();

        for r in 0..ROW_SIZE {
            let r = ROW_SIZE - r - 1;
            res += &Address::get_row_name(r).to_string();
            res += " ";

            for c in 0..ROW_SIZE {
                let cell = self.get_cell(Address::new(r ,c));

                if let Some(ref piece) = *cell {
                    let s = piece.to_string();
                    res += s.as_ref();
                    res += " ";
                } else {
                    res += ". ";
                };
            }
            res += "\n";
        }

        res += "  ";

        for c in 0..ROW_SIZE {
            res += &Address::get_col_name(c).to_string();
            res += " ";
        }

        write!(f, "{}", res)
    }
}