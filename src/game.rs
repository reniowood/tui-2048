use crate::board::Board;

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
    self.board.has_block_with(self.goal)
  }

  pub fn lose(&self) -> bool {
    self.board.has_empty_block()
  }

  pub fn move_up(mut self) {
    if self.board.try_to_move_up() {
      self.board.put_new_block();
    }
  }

  pub fn move_down(mut self) {
    if self.board.try_to_move_down() {
      self.board.put_new_block();
    }
  }

  pub fn move_left(mut self) {
    if self.board.try_to_move_left() {
      self.board.put_new_block();
    }
  }

  pub fn move_right(mut self) {
    if self.board.try_to_move_right() {
      self.board.put_new_block();
    }
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