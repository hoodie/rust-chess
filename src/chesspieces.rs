//! Predefined constant ChessPieces
use piece::{Suit, Color, Piece};

///♚
pub const BL_KING:Piece =
Piece{ sym: '♚', color: Color::Black, piece: Suit::King};
///♛
pub const BL_QUEEN:Piece =
Piece{ sym: '♛', color: Color::Black, piece: Suit::Queen};
///♜
pub const BL_ROOK:Piece =
Piece{ sym: '♜', color: Color::Black, piece: Suit::Rook};
///♝
pub const BL_BISHOP:Piece =
Piece{ sym: '♝', color: Color::Black, piece: Suit::Bishop};
///♞
pub const BL_KNIGHT:Piece =
Piece{ sym: '♞', color: Color::Black, piece: Suit::Knight};
///♟
pub const BL_PAWN:Piece =
Piece{ sym: '♟', color: Color::Black, piece: Suit::Pawn};
///♔
pub const WH_KING:Piece =
Piece{ sym: '♔', color: Color::White, piece: Suit::King};
///♕
pub const WH_QUEEN:Piece =
Piece{ sym: '♕', color: Color::White, piece: Suit::Queen};
///♖
pub const WH_ROOK:Piece =
Piece{ sym: '♖', color: Color::White, piece: Suit::Rook};
///♗
pub const WH_BISHOP:Piece =
Piece{ sym: '♗', color: Color::White, piece: Suit::Bishop};
///♘
pub const WH_KNIGHT:Piece =
Piece{ sym: '♘', color: Color::White, piece: Suit::Knight};
///♙
pub const WH_PAWN:Piece =
Piece{ sym: '♙', color: Color::White, piece: Suit::Pawn};
