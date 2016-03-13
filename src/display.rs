use std::fmt;

use gamestate::*;
use piece::{Suit, Color, Piece};


impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x = "ABCDEFGH".chars().collect::<Vec<char>>();
        write!(f, "{col}{row}", col= x[self.x as usize], row=self.y+1)
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{color} {piece}", piece=self.piece, color=self.color)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Color::White  => write!(f, "White"),
            Color::Black  => write!(f, "Black")
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.ty {
            MoveType::Move    => write!(f, "  {from} -> {to} ({note})", from=self.from, to=self.to, note=self.note),
            MoveType::Capture => write!(f, "X {from} -> {to} ({note})", from=self.from, to=self.to, note=self.note)
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Suit::King   => write!(f, "King"),
            Suit::Queen  => write!(f, "Queen"),
            Suit::Rook   => write!(f, "Rook"),
            Suit::Bishop => write!(f, "Bishop"),
            Suit::Knight => write!(f, "Knight"),
            Suit::Pawn   => write!(f, "Pawn")
        }
    }
}
