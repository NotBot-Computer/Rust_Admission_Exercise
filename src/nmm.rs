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


#[derive(Clone)]
struct Snapshot {
    board: [Option<Piece>; 24],
    to_move: Player,
    unplaced: [u8; 2],
    removed: [u8; 2],
    must_remove: Option<Player>,
}

pub struct Game {
    board: [Option<Piece>; 24],
    to_move: Player,
    unplaced: [u8; 2],
    removed: [u8; 2],
    must_remove: Option<Player>,
    history: Vec<Snapshot>,
}

impl Game {
    const INVALID: Point = 24;

    // tüm olası değirmenler (20 adet)
    const MILLS: [[Point; 3]; 16] = [
        [0, 1, 2],
        [2, 3, 4],
        [4, 5, 6],
        [6, 7, 0],
        [8, 9, 10],
        [10, 11, 12],
        [12, 13, 14],
        [14, 15, 8],
        [16, 17, 18],
        [18, 19, 20],
        [20, 21, 22],
        [22, 23, 16],
        [1, 9, 17],
        [3, 11, 19],
        [5, 13, 21],
        [7, 15, 23],
    ];

    // her noktanın komşuları (max 4, fazlalar INVALID)
    const NEIGHBORS: [[Point; 4]; 24] = [
        [1, 7, Game::INVALID, Game::INVALID],     // 0
        [0, 2, 9, Game::INVALID],     // 1
        [1, 3, Game::INVALID, Game::INVALID],    // 2
        [2, 4, 11, Game::INVALID],    // 3
        [3, 5, Game::INVALID, Game::INVALID],    // 4
        [4, 6, 13, Game::INVALID],    // 5
        [5, 7, Game::INVALID, Game::INVALID],    // 6
        [0, 6, 15, Game::INVALID],    // 7
        [Game::INVALID, 9, 15, 16],               // 8
        [1, 8, 10, 17],               // 9
        [Game::INVALID, 9, 11, 18],               // 10
        [3, 10, 12, 19],              // 11
        [Game::INVALID, 11, 13, 20],              // 12
        [5, 12, 14, 21],              // 13
        [Game::INVALID, 13, 15, 22],              // 14
        [7, 8, 14, 23],               // 15
        [Game::INVALID, 17, 23, Game::INVALID],   // 16
        [9, 16, 18, Game::INVALID],   // 17
        [Game::INVALID, 17, 19, Game::INVALID],  // 18
        [11, 18, 20, Game::INVALID],  // 19
        [Game::INVALID, 19, 21, Game::INVALID],  // 20
        [13, 20, 22, Game::INVALID],  // 21
        [Game::INVALID, 21, 23, Game::INVALID],  // 22
        [15, 16, 22, Game::INVALID],  // 23
    ];

    fn color_idx(c: Color) -> usize {
        match c {
            Color::White => 0,
            Color::Black => 1,
        }
    }

    fn snapshot(&self) -> Snapshot {
        Snapshot {
            board: self.board,
            to_move: self.to_move,
            unplaced: self.unplaced,
            removed: self.removed,
            must_remove: self.must_remove,
        }
    }

    fn forms_mill(&self, point: Point, color: Color) -> bool {
        for mill in &Self::MILLS {
            if mill.contains(&point) {
                if self.board[mill[0]] == Some(color)
                    && self.board[mill[1]] == Some(color)
                    && self.board[mill[2]] == Some(color)
                {
                    return true;
                }
            }
        }
        false
    }

    fn point_in_mill(&self, point: Point) -> bool {
        if let Some(color) = self.board[point] {
            self.forms_mill(point, color)
        } else {
            false
        }
    }

    fn all_pieces_in_mills(&self, color: Color) -> bool {
        for i in 0..24 {
            if self.board[i] == Some(color) && !self.point_in_mill(i) {
                return false;
            }
        }
        true
    }

    fn count_pieces(&self, color: Color) -> u8 {
        self.board
            .iter()
            .filter(|p| **p == Some(color))
            .count() as u8
    }

    fn are_adjacent(from: Point, to: Point) -> bool {
        Self::NEIGHBORS[from].iter().any(|&n| n == to)
    }

    // oyuncunun şu anda YASAL hamlesi var mı?
    fn player_can_move(&self, player: Player) -> bool {
        let idx = Self::color_idx(player);

        // önce: yerleştirme fazında mı?
        if self.unplaced[idx] > 0 {
            // boş yer varsa oynayabilir
            return self.board.iter().any(|p| p.is_none());
        }

        // artık hareket fazında
        let pieces = self.count_pieces(player);

        // uçma durumu: 3 tas kaldiysa herhangi bos yere gidebilir
        if pieces == 3 {
            // tahtada kendi tasi varsa ve bos yer varsa, hamle var demektir
            let has_own = self.board.iter().any(|p| *p == Some(player));
            let has_empty = self.board.iter().any(|p| p.is_none());
            return has_own && has_empty;
        }

        // normal hareket: komsusuna gidebilmeli
        for from in 0..24 {
            if self.board[from] == Some(player) {
                for &n in Self::NEIGHBORS[from].iter() {
                    if n < 24 && self.board[n].is_none() {
                        return true;
                    }
                }
            }
        }

        false
    }
}

impl NmmGame for Game {
    fn new() -> Self {
        Game {
            board: [None; 24],
            to_move: Player::White,
            unplaced: [9, 9],
            removed: [0, 0],
            must_remove: None,
            history: Vec::new(),
        }
    }

    fn action(&mut self, action: Action) -> Result<(), &'static str> {
        // once noktalar gecerli mi diye bakalim
        let check_point = |p: Point| -> Result<(), &'static str> {
            if p >= 24 {
                Err("Point out of range")
            } else {
                Ok(())
            }
        };

        // eger birinin tas sokmesi gerekiyorsa
        if let Some(waiting) = self.must_remove {
            // bu hamle remove olmali ve yapan da o olmali
            if action.player != waiting {
                return Err("This player must remove");
            }
            match action.action {
                ActionKind::Remove(p) => {
                    check_point(p)?;
                    // snapshot
                    self.history.push(self.snapshot());

                    let opponent = action.player.opposite();
                    if self.board[p] != Some(opponent) {
                        return Err("Can only remove opponent piece");
                    }

                    // eger rakibin mill disi tasi varsa milldekini sokemez
                    if !self.all_pieces_in_mills(opponent) && self.point_in_mill(p) {
                        // snapshot'i geri almak gerekir mi? Burada err'e düşmeden önce push ettik.
                        // kolay yol: en basta push etmemekti, ama simdi basitçe sonu geri cekelim.
                        self.history.pop();
                        return Err("Cannot remove a piece in a mill");
                    }

                    self.board[p] = None;
                    let opp_idx = Game::color_idx(opponent);
                    self.removed[opp_idx] += 1;
                    self.must_remove = None;
                    self.to_move = opponent;
                    Ok(())
                }
                _ => Err("Must remove a piece"),
            }
        } else {
            // normal sıra kontrolü
            if action.player != self.to_move {
                return Err("Not this player's turn");
            }

            let idx = Game::color_idx(action.player);
            match action.action {
                ActionKind::Place(p) => {
                    check_point(p)?;
                    if self.unplaced[idx] == 0 {
                        return Err("No pieces left to place");
                    }
                    if self.board[p].is_some() {
                        return Err("Point already occupied");
                    }

                    // snapshot
                    self.history.push(self.snapshot());

                    self.board[p] = Some(action.player);
                    self.unplaced[idx] -= 1;

                    if self.forms_mill(p, action.player) {
                        // Check if player can actually remove any piece
                        let opponent = action.player.opposite();
                        let all_opponent_in_mills = self.all_pieces_in_mills(opponent);
                        let can_remove = (0..24).any(|i| {
                            if self.board[i] == Some(opponent) {
                                all_opponent_in_mills || !self.point_in_mill(i)
                            } else {
                                false
                            }
                        });
                        if can_remove {
                            self.must_remove = Some(action.player);
                        } else {
                            // Can't remove, so continue the game
                            self.to_move = action.player.opposite();
                        }
                    } else {
                        self.to_move = action.player.opposite();
                    }
                    Ok(())
                }
                ActionKind::Move(from, to) => {
                    check_point(from)?;
                    check_point(to)?;

                    if self.unplaced[idx] > 0 {
                        return Err("Must place all pieces before moving");
                    }
                    if self.board[from] != Some(action.player) {
                        return Err("No piece of this player at source");
                    }
                    if self.board[to].is_some() {
                        return Err("Destination not empty");
                    }

                    let flying = self.count_pieces(action.player) == 3;
                    if !flying && !Game::are_adjacent(from, to) {
                        return Err("Points not adjacent");
                    }

                    // snapshot
                    self.history.push(self.snapshot());

                    self.board[from] = None;
                    self.board[to] = Some(action.player);

                    if self.forms_mill(to, action.player) {
                        // Check if player can actually remove any piece
                        let opponent = action.player.opposite();
                        let all_opponent_in_mills = self.all_pieces_in_mills(opponent);
                        let can_remove = (0..24).any(|i| {
                            if self.board[i] == Some(opponent) {
                                all_opponent_in_mills || !self.point_in_mill(i)
                            } else {
                                false
                            }
                        });
                        if can_remove {
                            self.must_remove = Some(action.player);
                        } else {
                            // Can't remove, so continue the game
                            self.to_move = action.player.opposite();
                        }
                    } else {
                        self.to_move = action.player.opposite();
                    }

                    Ok(())
                }
                ActionKind::Remove(_) => {
                    Err("Remove not allowed now")
                }
            }
        }
    }

    fn undo(&mut self) -> Result<(), &'static str> {
        if let Some(snap) = self.history.pop() {
            self.board = snap.board;
            self.to_move = snap.to_move;
            self.unplaced = snap.unplaced;
            self.removed = snap.removed;
            self.must_remove = snap.must_remove;
            Ok(())
        } else {
            Err("No action to undo")
        }
    }

    fn points(&self) -> &[Option<Piece>; 24] {
        &self.board
    }

    fn winner(&self) -> Option<Player> {
        // 1) 7 taş sökülmüş mü?
        if self.removed[Game::color_idx(Color::Black)] >= 7 {
            return Some(Color::White);
        }
        if self.removed[Game::color_idx(Color::White)] >= 7 {
            return Some(Color::Black);
        }

        // 2) sıradaki oyuncu oynayamıyorsa
        if !self.player_can_move(self.to_move) {
            return Some(self.to_move.opposite());
        }

        None
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
