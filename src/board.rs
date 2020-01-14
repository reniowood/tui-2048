use rand::Rng;
use rand::seq::SliceRandom;

#[derive(PartialEq, Debug)]
pub struct Board {
  width: usize,
  height: usize,
  blocks: Vec<Vec<Option<u32>>>
}

impl Board {
  pub fn new(width: usize, height: usize) -> Board {
    Board {
      width: width, 
      height: height,
      blocks: vec![vec![None; width]; height],
    }
  }

  pub fn try_to_move_up(&mut self) -> bool {
    let mut success = false;

    for j in 0..self.width {
      for i in 0..self.height {
        let mut k = i;

        while k >= 1 && self.blocks[k - 1][j].is_none() {
          k -= 1;
        }

        if k >= 1 && self.blocks[k - 1][j] == self.blocks[i][j] {
          self.blocks[k - 1][j] = self.blocks[i][j].map(|x| x * 2);

          success = true;
          self.blocks[i][j] = None;
        } else if k != i {
          self.blocks[k][j] = self.blocks[i][j];
        
          success = true;
          self.blocks[i][j] = None;
        }
      }
    }

    success
  }

  pub fn try_to_move_down(&mut self) -> bool {
    let mut success = false;

    for j in 0..self.width {
      for i in (0..self.height).rev() {
        let mut k = i;

        while k + 1 < self.height && self.blocks[k + 1][j] == None {
          k += 1;
        }

        if k + 1 < self.height && self.blocks[k + 1][j] == self.blocks[i][j] {
          self.blocks[k + 1][j] = self.blocks[i][j].map(|x| x * 2);

          success = true;
          self.blocks[i][j] = None;
        } else if k != i {
          self.blocks[k][j] = self.blocks[i][j];

          success = true;
          self.blocks[i][j] = None;
        }
      }
    }

    success
  }

  pub fn try_to_move_left(&mut self) -> bool {
    let mut success = false;

    for i in 0..self.height {
      for j in 0..self.width {
        let mut k = j;

        while k >= 1 && self.blocks[i][k - 1] == None {
          k -= 1;
        }

        if k >= 1 && self.blocks[i][k - 1] == self.blocks[i][j] {
          self.blocks[i][k - 1] = self.blocks[i][j].map(|x| x * 2);

          success = true;
          self.blocks[i][j] = None;
        } else if k != j {
          self.blocks[i][k] = self.blocks[i][j];

          success = true;
          self.blocks[i][j] = None;
        }
      }
    }

    success
  }

  pub fn try_to_move_right(&mut self) -> bool {
    let mut success = false;

    for i in 0..self.height {
      for j in (0..self.width).rev() {
        let mut k = j;

        while k + 1 < self.width && self.blocks[i][k + 1] == None {
          k += 1;
        }

        if k + 1 < self.width && self.blocks[i][k + 1] == self.blocks[i][j] {
          self.blocks[i][k + 1] = self.blocks[i][j].map(|x| x * 2);

          success = true;
          self.blocks[i][j] = None;
        } else if k != j {
          self.blocks[i][k] = self.blocks[i][j];

          success = true;
          self.blocks[i][j] = None;
        }
      }
    }

    success
  }

  fn pick_empty_index(&self) -> Option<(usize, usize)> {
    let mut indexes = Vec::new();

    for i in 0..self.height {
      for j in 0..self.width {
        if self.blocks[i as usize][j as usize] == None {
          indexes.push((i, j));
        }
      }
    }

    let mut rng = rand::thread_rng();

    indexes.choose(&mut rng).map(|index| (index.0, index.1))
  }

  fn create_new_block(&self) -> u32 {
    let mut rng = rand::thread_rng();

    rng.gen_range(1, 3) * 2
  }

  pub fn has_empty_block(&self) -> bool {
    self.pick_empty_index().is_some()
  }

  pub fn put_new_block(&mut self) -> Option<(usize, usize, u32)> {
    let block = self.create_new_block();
    let index = self.pick_empty_index()?;

    self.blocks[index.0 as usize][index.1 as usize] = Some(block);
    Some((index.0, index.1, block))
  }

  pub fn has_block_with(&self, value: u32) -> bool {
    for row in &self.blocks {
      for block in row {
        if *block == Some(value) {
          return true;
        }
      }
    }

    return false;
  }

  pub fn print(&self) {
    for row in &self.blocks {
      for block in row {
        match block {
          Some(value) => print!("{:4}", value),
          None => print!("    ")
        }
        print!("|");
      }
      println!("")
    }
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