#[derive(Debug)]
pub struct Player { pub color: Color, pub direction: i32 }

#[derive(Clone,Copy,Debug)]
pub enum ChessPiece { King, Queen, Rook, Bishop, Knight, Pawn }

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Color { White , Black }

#[derive(Clone,Copy,Debug)]
pub struct Piece {
    pub sym: char,
    pub color: Color,
    pub piece:ChessPiece}

pub type Board = [[Field;8];8];

#[derive(Clone,Copy,Debug)]
pub enum Field { Empty,     Piece(Piece) }

impl Field
{
    pub fn char(self) -> char{
        match self {
            Field::Empty => '⬚',
            Field::Piece(p) => p.sym,
        }
    }
}

// ChessPieces
// {{{
pub const BL_KING   :Piece = Piece{ sym : '♚', color: Color::Black, piece: ChessPiece::King   };
pub const BL_QUEEN  :Piece = Piece{ sym : '♛', color: Color::Black, piece: ChessPiece::Queen  };
pub const BL_ROOK   :Piece = Piece{ sym : '♜', color: Color::Black, piece: ChessPiece::Rook   };
pub const BL_BISHOP :Piece = Piece{ sym : '♝', color: Color::Black, piece: ChessPiece::Bishop };
pub const BL_KNIGHT :Piece = Piece{ sym : '♞', color: Color::Black, piece: ChessPiece::Knight };
pub const BL_PAWN   :Piece = Piece{ sym : '♟', color: Color::Black, piece: ChessPiece::Pawn   };
pub const WH_KING   :Piece = Piece{ sym : '♔', color: Color::White, piece: ChessPiece::King   };
pub const WH_QUEEN  :Piece = Piece{ sym : '♕', color: Color::White, piece: ChessPiece::Queen  };
pub const WH_ROOK   :Piece = Piece{ sym : '♖', color: Color::White, piece: ChessPiece::Rook   };
pub const WH_BISHOP :Piece = Piece{ sym : '♗', color: Color::White, piece: ChessPiece::Bishop };
pub const WH_KNIGHT :Piece = Piece{ sym : '♘', color: Color::White, piece: ChessPiece::Knight };
pub const WH_PAWN   :Piece = Piece{ sym : '♙', color: Color::White, piece: ChessPiece::Pawn   };
// }}}

