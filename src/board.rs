use crate::action::Action;

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Board {
  width: u32,
  height: u32,
  blocks: Vec<Vec<Option<u32>>>
}

impl Board {
  pub fn new(width: u32, height: u32) -> Board {
    Board {
      width: width, 
      height: height,
      blocks: vec![vec![None; width as usize]; height as usize],
    }
  }

  pub fn move_blocks(&self, mv: &Action) {
  }

  fn eq(&self, other: &Board) -> bool {
    return self.width == other.width && self.height == other.height && self.blocks == other.blocks
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_Board_new() {
    let width = 4;
    let height = 4;
    let board = Board::new(width, height);

    assert_eq!(board.width, width);
    assert_eq!(board.height, height);
    assert_eq!(board.blocks.len(), height as usize);
    assert_eq!(board.blocks[0].len(), width as usize);
    
    for row in board.blocks {
      for block in row {
        assert_eq!(block, None);
      }
    }
  }
}