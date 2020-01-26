use crate::block::block;
use crate::board::Board;

pub struct Game {
    goal: u32,
    pub board: Board,
    pub width: usize,
    pub height: usize,
    pub score: u32,
}

impl Game {
    pub fn new(goal: u32, width: usize, height: usize) -> Game {
        let mut board = Board::new(width, height);

        board = Game::put_new_block(board);
        board = Game::put_new_block(board);

        Game {
            goal: goal,
            board: board,
            width: width,
            height: height,
            score: 0,
        }
    }

    fn put_new_block(board: Board) -> Board {
        if let Some((row, col)) = board.pick_empty_index() {
            let block = block::random();

            board.put_new_block(row, col, block)
        } else {
            board
        }
    }

    pub fn win(&self) -> bool {
        self.board.has_block_with(self.goal)
    }

    pub fn lose(&self) -> bool {
        !self.win() && !self.board.has_empty_block()
    }

    pub fn move_up(&mut self) {
        let board = self.board.try_to_move_up();

        if board.updated {
            self.board = Game::put_new_block(board);
        }
    }

    pub fn move_down(&mut self) {
        let board = self.board.try_to_move_down();

        if board.updated {
            self.board = Game::put_new_block(board);
        }
    }

    pub fn move_left(&mut self) {
        let board = self.board.try_to_move_left();

        if board.updated {
            self.board = Game::put_new_block(board);
        }
    }

    pub fn move_right(&mut self) {
        let board = self.board.try_to_move_right();

        if board.updated {
            self.board = Game::put_new_block(board);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let width = 4;
        let height = 4;
        let goal = 32;
        
        let game = Game::new(goal, width, height);

        assert_eq!(goal, game.goal);
        assert_eq!(width, game.width);
        assert_eq!(height, game.height);
        assert_eq!(0, game.score);

        let mut num_blocks = 0;
        for i in 0..height {
            for j in 0..width {
                if game.board.blocks[i][j] > 0 {
                    num_blocks += 1;
                }
            }
        }
        assert_eq!(2, num_blocks);
    }

    #[test]
    fn test_win_when_there_is_block_with_goal_value() {
        let width = 4;
        let height = 4;
        let goal = 32;
        let blocks = vec![
            vec![0, 4, 0, 32],
            vec![0, 2, 4, 0],
            vec![0, 0, 2, 0],
            vec![0, 0, 0, 0],
        ];
        let board = Board {
            width: width,
            height: height,
            blocks: blocks,
            updated: false
        };
        let game = Game {
            goal: goal,
            board: board,
            width: width,
            height: height,
            score: 0
        };

        assert!(game.win());
    }

    #[test]
    fn test_win_when_there_is_no_block_with_goal_value() {
        let width = 4;
        let height = 4;
        let goal = 32;
        let blocks = vec![
            vec![0, 4, 0, 31],
            vec![0, 2, 4, 0],
            vec![0, 0, 2, 0],
            vec![0, 0, 0, 0],
        ];
        let board = Board {
            width: width,
            height: height,
            blocks: blocks,
            updated: false
        };
        let game = Game {
            goal: goal,
            board: board,
            width: width,
            height: height,
            score: 0
        };

        assert!(!game.win());
    }

    #[test]
    fn test_lose_when_board_is_full() {
        let width = 4;
        let height = 4;
        let goal = 32;
        let blocks = vec![
            vec![2, 4, 8, 31],
            vec![4, 2, 4, 2],
            vec![8, 4, 2, 4],
            vec![2, 8, 4, 2],
        ];
        let board = Board {
            width: width,
            height: height,
            blocks: blocks,
            updated: false
        };
        let game = Game {
            goal: goal,
            board: board,
            width: width,
            height: height,
            score: 0
        };

        assert!(game.lose());
    }

    #[test]
    fn test_lose_when_board_has_empty_block() {
        let width = 4;
        let height = 4;
        let goal = 32;
        let blocks = vec![
            vec![0, 4, 2, 31],
            vec![4, 2, 4, 2],
            vec![2, 4, 2, 4],
            vec![4, 2, 4, 2],
        ];
        let board = Board {
            width: width,
            height: height,
            blocks: blocks,
            updated: false
        };
        let game = Game {
            goal: goal,
            board: board,
            width: width,
            height: height,
            score: 0
        };

        assert!(!game.lose());
    }

    #[test]
    fn test_lose_when_board_is_full_but_has_goal_value() {
        let width = 4;
        let height = 4;
        let goal = 32;
        let blocks = vec![
            vec![32, 4, 2, 31],
            vec![4, 2, 4, 2],
            vec![2, 4, 2, 4],
            vec![4, 2, 4, 2],
        ];
        let board = Board {
            width: width,
            height: height,
            blocks: blocks,
            updated: false
        };
        let game = Game {
            goal: goal,
            board: board,
            width: width,
            height: height,
            score: 0
        };

        assert!(!game.lose());
    }
}