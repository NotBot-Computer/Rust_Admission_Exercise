// Board representation, game logic, and public API for Nine Men's Morris
// This module will be tested from the ./tests folder
// Rules (incl. 'flying'): https://en.wikipedia.org/wiki/Nine_men%27s_morris
// White begins

use std::{fmt::Display, str::FromStr};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn opposite(self) -> Color {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

pub type Player = Color;
pub type Piece = Color;
/// The board is represented by 24 points, numbered as follows:
/// 0––––––––1 –––––––2
/// |  8–––––9 ––––10 |
/// |  |  16–17–18 |  |
/// 7 –15–23    19–11–3
/// |  |  22–21–20 |  |
/// |  14––––13––––12 |
/// 6––––––––5 –––––––4
pub type Point = usize; // 0–23

/// Describes the contents of an action.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ActionKind {
    Place(Point),
    Move(Point, Point),
    Remove(Point),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Action {
    pub player: Player,
    pub action: ActionKind,
}

// This implementation is used extensively for testing
impl FromStr for Action {
    type Err = &'static str;

    /// Example inputs:
    /// "W P 0" - White places at 0
    /// "B M 0 1" - Black moves from 0 to 1
    /// "W R 5" - White removes at 5
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() < 3 {
            return Err("Invalid action format");
        }
        let player = match parts[0] {
            "W" => Player::White,
            "B" => Player::Black,
            _ => return Err("Invalid player"),
        };
        let action = match parts[1] {
            "P" => {
                let point: Point = parts[2].parse().map_err(|_| "Invalid point")?;
                ActionKind::Place(point)
            }
            "M" => {
                if parts.len() != 4 {
                    return Err("Invalid move format");
                }
                let from: Point = parts[2].parse().map_err(|_| "Invalid from point")?;
                let to: Point = parts[3].parse().map_err(|_| "Invalid to point")?;
                ActionKind::Move(from, to)
            }
            "R" => {
                let point: Point = parts[2].parse().map_err(|_| "Invalid point")?;
                ActionKind::Remove(point)
            }
            _ => return Err("Invalid action type"),
        };
        Ok(Action { player, action })
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let player_str = match self.player {
            Player::White => "W",
            Player::Black => "B",
        };
        let action_str = match self.action {
            ActionKind::Place(p) => format!("P {p}"),
            ActionKind::Move(from, to) => format!("M {from} {to}"),
            ActionKind::Remove(p) => format!("R {p}"),
        };
        write!(f, "{player_str} {action_str}")
    }
}

pub trait NmmGame {
    /// Creates a new instance with an empty board.
    fn new() -> Self;
    /// Applies the given action.
    fn action(&mut self, action: Action) -> Result<(), &'static str>;
    /// Undoes the last action.
    /// This should fail if there is no last action to be undone.
    fn undo(&mut self) -> Result<(), &'static str>;
    /// All poinst of the game board
    fn points(&self) -> &[Option<Piece>; 24];
    /// Returns if there is currently a winner.
    /// There are two win-conditions:
    /// - one player has removed 7 pieces of the opponent
    /// - one player cannot make a legal move
    fn winner(&self) -> Option<Player>;
}

/*
Complete the struct called `Game` that implements the `NmmGame` trait. All functionality exposed by
the trait should be implemented.
*/

pub struct Game {/* add your fields here */}

impl NmmGame for Game {
    fn new() -> Self {
        unimplemented!()
    }

    fn action(&mut self, _action: Action) -> Result<(), &'static str> {
        unimplemented!()
    }

    fn undo(&mut self) -> Result<(), &'static str> {
        unimplemented!()
    }

    fn points(&self) -> &[Option<Piece>; 24] {
        unimplemented!()
    }

    fn winner(&self) -> Option<Player> {
        unimplemented!()
    }
}

// For grading this assignment, the tests in the `tests` folder will be used.
// Small unit tests are generally included in the same file as the code they test.
// You are free to add more tests here if you wish.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_new_is_empty() {
        let game = Game::new();
        for pos in *game.points() {
            assert_eq!(pos, None);
        }
    }
}
