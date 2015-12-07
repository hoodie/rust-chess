//! Describes a single player.
//!
//! A player has a color and a direction to move to.

use piece::Color;

#[derive(Debug,PartialEq,Eq,Hash,Clone)]
pub struct Player {
    pub color: Color,
    pub direction: i8
}

impl Player{
    pub fn new(color:Color, direction:i8) -> Player{
        assert_eq!(direction.abs(), 1);
        Player {
            color: color,
            direction: direction
        }
    }
}
