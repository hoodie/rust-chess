//! Definition of Chesspieces, colors and "suits".
//!
//! That's it here.

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Suit { King, Queen, Rook, Bishop, Knight, Pawn }

#[derive(Clone,Copy,Debug,Hash,Eq,PartialEq)]
pub enum Color { White , Black }

#[derive(Clone,Copy,Debug)]
pub struct Piece {
    /// Holds the character displayed for this piece
    pub sym: char,
    /// Black or white
    pub color: Color,
    /// ChessPieceType of the Chesspiece
    pub piece:Suit
}
