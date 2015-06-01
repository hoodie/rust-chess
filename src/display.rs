use board::*;
use std::fmt;

impl fmt::Display for Point
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x = "ABCDEFGH".chars().collect::<Vec<char>>();
        write!(f, "({col}{row})", col= x[self.x as usize], row=self.y+1)
    }
}

impl fmt::Display for Piece
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{color} {piece}", piece=self.piece, color=self.color)
    }
}

impl fmt::Display for Color
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use board::Color::*;
        match self {
            &White  => write!(f, "White"),
            &Black  => write!(f, "Black")
        }
    }
}

impl fmt::Display for ChessPiece
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use board::ChessPiece::*;
        match self {
            &King   => write!(f, "King"),
            &Queen  => write!(f, "Queen"),
            &Rook   => write!(f, "Rook"),
            &Bishop => write!(f, "Bishop"),
            &Knight => write!(f, "Knight"),
            &Pawn   => write!(f, "Pawn")
        }
    }
}
