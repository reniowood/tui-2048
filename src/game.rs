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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_Game_new() {
    let goal = 2048;
    let width = 4;
    let height = 4;
    let game = Game::new(goal, width, height);

    assert_eq!(game.goal, goal);
    assert_eq!(game.board, Board::new(width, height));
  }
}