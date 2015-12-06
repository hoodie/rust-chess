#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Suit { King, Queen, Rook, Bishop, Knight, Pawn }

#[derive(Clone,Copy,Debug,Hash,Eq,PartialEq)]
pub enum Color { White , Black }

#[derive(Clone,Copy,Debug)]
pub struct Piece {
    pub sym: char,
    pub color: Color,
    pub piece:Suit
}
