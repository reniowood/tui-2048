use rand::seq::SliceRandom;

#[derive(PartialEq, Debug, Clone)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub blocks: Vec<Vec<u32>>,
    pub updated: bool
}

impl Board {
    pub fn new(width: usize, height: usize) -> Board {
        Board {
            width: width,
            height: height,
            blocks: vec![vec![0; width]; height],
            updated: false
        }
    }

    pub fn try_to_move_up(&self) -> Board {
        let mut board = Board::new(self.width, self.height);
        let mut merged = vec![vec![false; self.width]; self.height];

        for j in 0..self.width {
            let mut k = 0;

            for i in 0..self.height {
                if self.blocks[i][j] > 0 {
                    if k == 0 || board.blocks[k - 1][j] != self.blocks[i][j] || merged[k - 1][j] {
                        board.blocks[k][j] = self.blocks[i][j];
                        if k != i {
                            board.updated = true;
                        }
                        k += 1;
                    } else {
                        board.blocks[k - 1][j] = board.blocks[k - 1][j] * 2;
                        merged[k - 1][j] = true;
                        board.updated = true;
                    }
                }
            }
        }

        board
    }

    pub fn try_to_move_down(&self) -> Board {
        let mut board = Board::new(self.width, self.height);
        let mut merged = vec![vec![false; self.width]; self.height];

        for j in 0..self.width {
            let mut k = self.height - 1;

            for i in (0..self.height).rev() {
                if self.blocks[i][j] > 0 {
                    if k == self.height - 1 || board.blocks[k + 1][j] != self.blocks[i][j] || merged[k + 1][j] {
                        board.blocks[k][j] = self.blocks[i][j];
                        if k != i {
                            board.updated = true;
                        }
                        if k > 0 {
                            k -= 1;
                        }
                    } else {
                        board.blocks[k + 1][j] = board.blocks[k + 1][j] * 2;
                        merged[k + 1][j] = true;
                        board.updated = true;
                    }
                }
            }
        }

        board
    }

    pub fn try_to_move_left(&self) -> Board {
        let mut board = Board::new(self.width, self.height);
        let mut merged = vec![vec![false; self.width]; self.height];

        for i in 0..self.height {
            let mut k = 0;

            for j in 0..self.width {
                if self.blocks[i][j] > 0 {
                    if k == 0 || board.blocks[i][k - 1] != self.blocks[i][j] || merged[i][k - 1] {
                        board.blocks[i][k] = self.blocks[i][j];
                        if k != j {
                            board.updated = true;
                        }
                        k += 1;
                    } else {
                        board.blocks[i][k - 1] = board.blocks[i][k - 1] * 2;
                        merged[i][k - 1] = true;
                        board.updated = true;
                    }
                }
            }
        }

        board
    }

    pub fn try_to_move_right(&self) -> Board {
        let mut board = Board::new(self.width, self.height);
        let mut merged = vec![vec![false; self.width]; self.height];

        for i in 0..self.height {
            let mut k = self.width - 1;

            for j in (0..self.width).rev() {
                if self.blocks[i][j] > 0 {
                    if k == self.width - 1 || board.blocks[i][k + 1] != self.blocks[i][j] || merged[i][k + 1] {
                        board.blocks[i][k] = self.blocks[i][j];
                        if k != j {
                            board.updated = true;
                        }
                        if k > 0 {
                            k -= 1;
                        }
                    } else {
                        board.blocks[i][k + 1] = board.blocks[i][k + 1] * 2;
                        merged[i][k + 1] = true;
                        board.updated = true;
                    }
                }
            }
        }

        board
    }

    pub fn pick_empty_index(&self) -> Option<(usize, usize)> {
        let mut indexes = Vec::new();

        for i in 0..self.height {
            for j in 0..self.width {
                if self.blocks[i][j] == 0 {
                    indexes.push((i, j));
                }
            }
        }

        let mut rng = rand::thread_rng();

        indexes.choose(&mut rng).map(|index| (index.0, index.1))
    }

    pub fn has_empty_block(&self) -> bool {
        self.pick_empty_index().is_some()
    }

    pub fn put_new_block(self, row: usize, col: usize, value: u32) -> Board {
        let mut board = self.clone();

        board.blocks[row][col] = value;

        board
    }

    pub fn has_block_with(&self, value: u32) -> bool {
        for row in &self.blocks {
            for block in row {
                if *block == value {
                    return true;
                }
            }
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let width = 4;
        let height = 4;
        let board = Board::new(width, height);

        assert_eq!(board.width, width);
        assert_eq!(board.height, height);
        assert_eq!(board.blocks.len(), height as usize);
        assert_eq!(board.blocks[0].len(), width as usize);

        for row in board.blocks {
            for block in row {
                assert_eq!(block, 0);
            }
        }
    }

    #[test]
    fn test_try_to_move_up_merge() {
        let blocks = vec![
            vec![0, 4, 4, 0],
            vec![0, 2, 4, 0],
            vec![0, 0, 2, 0],
            vec![0, 0, 0, 0],
        ];
        let expected_blocks = vec![
            vec![0, 4, 8, 0],
            vec![0, 2, 2, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];

        let board = Board {
            width: 4,
            height: 4,
            blocks: blocks,
            updated: false
        };
        let next_board = board.try_to_move_up();

        assert!(next_board.updated);
        assert_eq!(next_board.blocks, expected_blocks);
    }

    #[test]
    fn test_try_to_move_up_merge_only_once() {
        let blocks = vec![
            vec![0, 4, 4, 0],
            vec![0, 2, 4, 0],
            vec![0, 0, 4, 0],
            vec![0, 0, 4, 0],
        ];
        let expected_blocks = vec![
            vec![0, 4, 8, 0],
            vec![0, 2, 8, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];

        let board = Board {
            width: 4,
            height: 4,
            blocks: blocks,
            updated: false
        };
        let next_board = board.try_to_move_up();

        assert!(next_board.updated);
        assert_eq!(next_board.blocks, expected_blocks);
    }

    #[test]
    fn test_try_to_move_up_move() {
        let blocks = vec![
            vec![0, 4, 0, 0],
            vec![0, 2, 4, 0],
            vec![0, 0, 2, 0],
            vec![0, 0, 0, 0],
        ];
        let expected_blocks = vec![
            vec![0, 4, 4, 0],
            vec![0, 2, 2, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];

        let board = Board {
            width: 4,
            height: 4,
            blocks: blocks,
            updated: false
        };
        let next_board = board.try_to_move_up();

        assert!(next_board.updated);
        assert_eq!(next_board.blocks, expected_blocks);
    }

    #[test]
    fn test_try_to_move_up_not_updated() {
        let blocks = vec![
            vec![0, 4, 4, 0],
            vec![0, 2, 2, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];
        let expected_blocks = vec![
            vec![0, 4, 4, 0],
            vec![0, 2, 2, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];

        let board = Board {
            width: 4,
            height: 4,
            blocks: blocks,
            updated: false
        };
        let next_board = board.try_to_move_up();

        assert!(!next_board.updated);
        assert_eq!(next_board.blocks, expected_blocks);
    }

    #[test]
    fn test_try_to_move_down_merge() {
        let blocks = vec![
            vec![0, 4, 4, 0],
            vec![2, 2, 4, 0],
            vec![0, 4, 2, 0],
            vec![0, 0, 0, 0],
        ];
        let expected_blocks = vec![
            vec![0, 0, 0, 0],
            vec![0, 4, 0, 0],
            vec![0, 2, 8, 0],
            vec![2, 4, 2, 0],
        ];

        let board = Board {
            width: 4,
            height: 4,
            blocks: blocks,
            updated: false
        };
        let next_board = board.try_to_move_down();

        assert!(next_board.updated);
        assert_eq!(next_board.blocks, expected_blocks);
    }

    #[test]
    fn test_try_to_move_down_merge_only_once() {
        let blocks = vec![
            vec![0, 4, 4, 0],
            vec![2, 2, 4, 0],
            vec![0, 4, 4, 0],
            vec![0, 0, 4, 0],
        ];
        let expected_blocks = vec![
            vec![0, 0, 0, 0],
            vec![0, 4, 0, 0],
            vec![0, 2, 8, 0],
            vec![2, 4, 8, 0],
        ];

        let board = Board {
            width: 4,
            height: 4,
            blocks: blocks,
            updated: false
        };
        let next_board = board.try_to_move_down();

        assert!(next_board.updated);
        assert_eq!(next_board.blocks, expected_blocks);
    }

    #[test]
    fn test_try_to_move_down_move() {
        let blocks = vec![
            vec![0, 0, 0, 0],
            vec![0, 4, 4, 0],
            vec![0, 0, 2, 0],
            vec![0, 0, 4, 2],
        ];
        let expected_blocks = vec![
            vec![0, 0, 0, 0],
            vec![0, 0, 4, 0],
            vec![0, 0, 2, 0],
            vec![0, 4, 4, 2],
        ];

        let board = Board {
            width: 4,
            height: 4,
            blocks: blocks,
            updated: false
        };
        let next_board = board.try_to_move_down();

        assert!(next_board.updated);
        assert_eq!(next_board.blocks, expected_blocks);
    }

    #[test]
    fn test_try_to_move_down_not_updated() {
        let blocks = vec![
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 4, 4, 0],
            vec![0, 2, 2, 0],
        ];
        let expected_blocks = vec![
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 4, 4, 0],
            vec![0, 2, 2, 0],
        ];

        let board = Board {
            width: 4,
            height: 4,
            blocks: blocks,
            updated: false
        };
        let next_board = board.try_to_move_down();

        assert!(!next_board.updated);
        assert_eq!(next_board.blocks, expected_blocks);
    }

    #[test]
    fn test_try_to_move_down_when_column_is_full() {
        let blocks = vec![
            vec![4, 0, 0, 0],
            vec![2, 0, 0, 0],
            vec![8, 0, 0, 0],
            vec![16, 0, 0, 0],
        ];
        let expected_blocks = vec![
            vec![4, 0, 0, 0],
            vec![2, 0, 0, 0],
            vec![8, 0, 0, 0],
            vec![16, 0, 0, 0],
        ];

        let board = Board {
            width: 4,
            height: 4,
            blocks: blocks,
            updated: false
        };
        let next_board = board.try_to_move_down();

        assert!(!next_board.updated);
        assert_eq!(next_board.blocks, expected_blocks);
    }

    #[test]
    fn test_try_to_move_left_merge() {
        let blocks = vec![
            vec![4, 4, 0, 0],
            vec![4, 0, 0, 0],
            vec![4, 2, 0, 0],
            vec![0, 0, 0, 0],
        ];
        let expected_blocks = vec![
            vec![8, 0, 0, 0],
            vec![4, 0, 0, 0],
            vec![4, 2, 0, 0],
            vec![0, 0, 0, 0],
        ];

        let board = Board {
            width: 4,
            height: 4,
            blocks: blocks,
            updated: false
        };
        let next_board = board.try_to_move_left();

        assert!(next_board.updated);
        assert_eq!(next_board.blocks, expected_blocks);
    }

    #[test]
    fn test_try_to_move_left_merge_only_once() {
        let blocks = vec![
            vec![0, 4, 4, 0],
            vec![2, 2, 4, 0],
            vec![0, 4, 2, 0],
            vec![0, 0, 0, 0],
        ];
        let expected_blocks = vec![
            vec![8, 0, 0, 0],
            vec![4, 4, 0, 0],
            vec![4, 2, 0, 0],
            vec![0, 0, 0, 0],
        ];

        let board = Board {
            width: 4,
            height: 4,
            blocks: blocks,
            updated: false
        };
        let next_board = board.try_to_move_left();

        assert!(next_board.updated);
        assert_eq!(next_board.blocks, expected_blocks);
    }

    #[test]
    fn test_try_to_move_left_move() {
        let blocks = vec![
            vec![2, 0, 0, 0],
            vec![0, 4, 0, 0],
            vec![0, 0, 2, 0],
            vec![0, 0, 4, 2],
        ];
        let expected_blocks = vec![
            vec![2, 0, 0, 0],
            vec![4, 0, 0, 0],
            vec![2, 0, 0, 0],
            vec![4, 2, 0, 0],
        ];

        let board = Board {
            width: 4,
            height: 4,
            blocks: blocks,
            updated: false
        };
        let next_board = board.try_to_move_left();

        assert!(next_board.updated);
        assert_eq!(next_board.blocks, expected_blocks);
    }

    #[test]
    fn test_try_to_move_left_not_updated() {
        let blocks = vec![
            vec![0, 0, 0, 0],
            vec![2, 0, 0, 0],
            vec![4, 2, 0, 0],
            vec![0, 0, 0, 0],
        ];
        let expected_blocks = vec![
            vec![0, 0, 0, 0],
            vec![2, 0, 0, 0],
            vec![4, 2, 0, 0],
            vec![0, 0, 0, 0],
        ];

        let board = Board {
            width: 4,
            height: 4,
            blocks: blocks,
            updated: false
        };
        let next_board = board.try_to_move_left();

        assert!(!next_board.updated);
        assert_eq!(next_board.blocks, expected_blocks);
    }

    #[test]
    fn test_try_to_move_right_merge() {
        let blocks = vec![
            vec![4, 4, 0, 0],
            vec![4, 0, 0, 0],
            vec![4, 2, 0, 0],
            vec![0, 0, 0, 0],
        ];
        let expected_blocks = vec![
            vec![0, 0, 0, 8],
            vec![0, 0, 0, 4],
            vec![0, 0, 4, 2],
            vec![0, 0, 0, 0],
        ];

        let board = Board {
            width: 4,
            height: 4,
            blocks: blocks,
            updated: false
        };
        let next_board = board.try_to_move_right();

        assert!(next_board.updated);
        assert_eq!(next_board.blocks, expected_blocks);
    }

    #[test]
    fn test_try_to_move_right_merge_only_once() {
        let blocks = vec![
            vec![0, 4, 4, 0],
            vec![2, 2, 4, 0],
            vec![0, 4, 2, 0],
            vec![0, 0, 0, 0],
        ];
        let expected_blocks = vec![
            vec![0, 0, 0, 8],
            vec![0, 0, 4, 4],
            vec![0, 0, 4, 2],
            vec![0, 0, 0, 0],
        ];

        let board = Board {
            width: 4,
            height: 4,
            blocks: blocks,
            updated: false
        };
        let next_board = board.try_to_move_right();

        assert!(next_board.updated);
        assert_eq!(next_board.blocks, expected_blocks);
    }

    #[test]
    fn test_try_to_move_right_move() {
        let blocks = vec![
            vec![2, 0, 0, 0],
            vec![0, 4, 0, 0],
            vec![0, 0, 2, 0],
            vec![0, 0, 4, 2],
        ];
        let expected_blocks = vec![
            vec![0, 0, 0, 2],
            vec![0, 0, 0, 4],
            vec![0, 0, 0, 2],
            vec![0, 0, 4, 2],
        ];

        let board = Board {
            width: 4,
            height: 4,
            blocks: blocks,
            updated: false
        };
        let next_board = board.try_to_move_right();

        assert!(next_board.updated);
        assert_eq!(next_board.blocks, expected_blocks);
    }

    #[test]
    fn test_try_to_move_right_not_updated() {
        let blocks = vec![
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 2],
            vec![0, 0, 4, 2],
            vec![0, 0, 0, 0],
        ];
        let expected_blocks = vec![
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 2],
            vec![0, 0, 4, 2],
            vec![0, 0, 0, 0],
        ];

        let board = Board {
            width: 4,
            height: 4,
            blocks: blocks,
            updated: false
        };
        let next_board = board.try_to_move_right();

        assert!(!next_board.updated);
        assert_eq!(next_board.blocks, expected_blocks);
    }

    #[test]
    fn test_try_to_move_right_when_row_is_full() {
        let blocks = vec![
            vec![4, 2, 8, 16],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];
        let expected_blocks = vec![
            vec![4, 2, 8, 16],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];

        let board = Board {
            width: 4,
            height: 4,
            blocks: blocks,
            updated: false
        };
        let next_board = board.try_to_move_right();

        assert!(!next_board.updated);
        assert_eq!(next_board.blocks, expected_blocks);
    }
}