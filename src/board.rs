use piece::{Suit, Color, Piece};

#[derive(Copy,Clone,Debug)]
pub struct Point {
    pub x:i8,
    pub y:i8
}

#[derive(Debug,Clone)]
pub struct Move {
    pub from: Point,
    pub to: Point,
    pub note:&'static str
}

#[derive(Clone,Copy,Debug)]
pub enum Field { Outside, Empty, Piece(Piece) }

impl Field {
    pub fn char(self) -> char{
        match self {
            Field::Outside  => '☠', //💀
            Field::Empty    => '⬚',
            Field::Piece(p) => p.sym,
        }
    }
}

// ChessPieces
// {{{
pub const BL_KING   :Piece = Piece{ sym : '♚', color: Color::Black, piece: Suit::King   };
pub const BL_QUEEN  :Piece = Piece{ sym : '♛', color: Color::Black, piece: Suit::Queen  };
pub const BL_ROOK   :Piece = Piece{ sym : '♜', color: Color::Black, piece: Suit::Rook   };
pub const BL_BISHOP :Piece = Piece{ sym : '♝', color: Color::Black, piece: Suit::Bishop };
pub const BL_KNIGHT :Piece = Piece{ sym : '♞', color: Color::Black, piece: Suit::Knight };
pub const BL_PAWN   :Piece = Piece{ sym : '♟', color: Color::Black, piece: Suit::Pawn   };
pub const WH_KING   :Piece = Piece{ sym : '♔', color: Color::White, piece: Suit::King   };
pub const WH_QUEEN  :Piece = Piece{ sym : '♕', color: Color::White, piece: Suit::Queen  };
pub const WH_ROOK   :Piece = Piece{ sym : '♖', color: Color::White, piece: Suit::Rook   };
pub const WH_BISHOP :Piece = Piece{ sym : '♗', color: Color::White, piece: Suit::Bishop };
pub const WH_KNIGHT :Piece = Piece{ sym : '♘', color: Color::White, piece: Suit::Knight };
pub const WH_PAWN   :Piece = Piece{ sym : '♙', color: Color::White, piece: Suit::Pawn   };
// }}}
