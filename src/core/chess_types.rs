use std::{fmt::Display, str::FromStr};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

#[derive(Debug)]
pub enum MoveType {
    March,
    Capture
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Address {
    pub row: u8,
    pub col: u8
}

impl Address {
    pub fn new(row: u8, col: u8) -> Self {
        assert!(row < ROW_SIZE);
        assert!(col < ROW_SIZE);

        Address { row, col }
    }

    pub fn get_row_name(row: u8) -> char {
        ('1' as u8 + row) as char
    }

    pub fn get_col_name(col: u8) -> char {
        ('a' as u8 + col) as char
    }

    pub fn get_color(&self) -> Color {
        let flip = self.row % 2;
        let is_black = (self.col % 2) == flip;
        
        if is_black { Color::Black } else { Color::White }
    }

    pub fn get_shifted(&self, row_offset: i8, col_offset: i8) -> Option<Address> {
        let new_row = (self.row as i8) + row_offset;
        let new_col = (self.col as i8) + col_offset;

        if new_row >= 0
        && new_row < (ROW_SIZE as i8)
        && new_col >= 0
        && new_col < (ROW_SIZE as i8) {
            Some(Address::new(new_row as u8, new_col as u8))
        } else {
            None
        }
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

        if let r @ '1'..='8' = row {
            res.row = (r as u8) - ('1' as u8)
        } else {
            return Err(ParseAddressError)
        }
        
        Ok(res)
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", Self::get_col_name(self.col), Self::get_row_name(self.row))
    }
}

pub struct Board {
    pub pieces: BoardLayer<Option<Piece>>,
    pub flip_board: bool
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
            ],
            flip_board: false
        }
    }

    pub fn new_empty() -> Self {
        Board {
            pieces: [None; CELLS_COUNT as usize],
            flip_board: false
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

    pub fn flip(&mut self) {
        self.flip_board = !self.flip_board;
    }

    pub fn flip_for(&mut self, color: Color) {
        self.flip_board = color == Color::Black;
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();
        res += "\n";

        for r in 0..ROW_SIZE {
            let r = if !self.flip_board {
                ROW_SIZE - r - 1
            } else {
                r
            };

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
            let c = if self.flip_board {
                ROW_SIZE - c - 1
            } else {
                c
            };

            res += &Address::get_col_name(c).to_string();
            res += " ";
        }

        write!(f, "{}", res)
    }
}

#[cfg(test)]
mod test {
    use std::{cell::Cell};

    use super::*;

    #[test]
    fn address_parse() {
        macro_rules! check_neg {
            ($addr:expr) => {
                assert_eq!(Address::from_str($addr), Err(ParseAddressError));
            };
        }

        check_neg!("");
        check_neg!("a");
        check_neg!("f11");
        check_neg!("6e");
        check_neg!("f9");
        check_neg!("j5");
        check_neg!("2");
        check_neg!("2789");
        check_neg!("1f");
        check_neg!("c0");

        for r in '1'..='8' {
            for c in 'a'..='h' {
                let addr_str = format!("{}{}", c, r);
                let addr = Address::from_str(&addr_str).unwrap();

                let r_int: u8 = (r as u8) - ('1' as u8);
                let c_int: u8 = (c as u8) - ('a' as u8);

                println!("{}: ({}, {})", addr_str, r_int, c_int);

                assert_eq!(addr, Address {row: r_int, col: c_int});
            }
        }
    }

    #[test]
    fn address_color() {
        let color = Cell::new(Color::Black);
        let flip_color = || {
            color.set(
                if color.get() == Color::Black { Color::White } else { Color::Black }
            );
        };

        for r in 0..ROW_SIZE {
            for c in 0..ROW_SIZE {
                let addr = Address::new(r, c);

                println!("{}: {:?}", addr, color.get());
                assert_eq!(addr.get_color(), color.get());

                flip_color();
            }
            flip_color();
        }
    }

    #[test]
    fn address_shift() {
        let cell = |s: &str| -> Address {
            Address::from_str(s).unwrap()
        };
        let addr = |row: u8, col :u8| -> Address {
            Address::new(row, col)
        };

        assert_eq!(
            cell("e4").get_shifted(-2, 0),
            Some(cell("e2"))
        );
        assert_eq!(
            addr(5, 5).get_shifted(1, 2),
            Some(addr(6, 7))
        );
        assert_eq!(
            cell("a1").get_shifted(0, 0),
            Some(cell("a1"))
        );
        assert_eq!(
            cell("a1").get_shifted(-1, 0),
            None
        );
        assert_eq!(
            cell("a1").get_shifted(0, -1),
            None
        );
        assert_eq!(
            cell("a1").get_shifted(-1, -1),
            None
        );
        assert_eq!(
            cell("g8").get_shifted(-1, -1),
            Some(cell("f7"))
        );
        assert_eq!(
            cell("g8").get_shifted(1, 0),
            None
        );
        assert_eq!(
            cell("g8").get_shifted(0, 1),
            Some(cell("h8"))
        );
        assert_eq!(
            cell("a8").get_shifted(-7, 7),
            Some(cell("h1"))
        );
    }

    #[test]
    fn board_index() {
        assert_eq!(Board::get_index(Address::from_str("e4").unwrap()), 28);

        let mut index = 0;

        for r in 0..ROW_SIZE {
            for c in 0..ROW_SIZE {
                let addr = Address::new(r, c);

                println!("{}: {}", addr, index);
                assert_eq!(Board::get_index(addr), index);

                index += 1;
            }
        }
    }
}