//! Very simple chess board.
//!
#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

#![feature(alloc_system)]
extern crate alloc_system;
pub mod piece;
pub mod gamestate;
pub mod chesspieces;
mod display;
