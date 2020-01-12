use crate::board::Board;
use crate::action::Action;

pub struct Game {
  goal: u32,
  board: Board,
}

impl Game {
  pub fn new(goal: u32, width: u32, height: u32) -> Game {
    Game {
      goal: goal,
      board: Board::new(width, height),
    }
  }

  pub fn win(&self) -> bool {
    false
  }

  pub fn lose(&self) -> bool {
    false
  }

  pub fn play(&self, mv: &Action) {
  }
}