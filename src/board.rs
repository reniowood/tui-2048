use crate::action::Action;

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
}