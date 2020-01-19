use crate::board::Board;

pub struct Game {
    goal: u32,
    pub board: Board,
    pub width: usize,
    pub height: usize,
}

impl Game {
    pub fn new(goal: u32, width: usize, height: usize) -> Game {
        let mut board = Board::new(width, height);

        board.put_new_block();
        board.put_new_block();

        Game {
            goal: goal,
            board: board,
            width: width,
            height: height,
        }
    }

    pub fn win(&self) -> bool {
        self.board.has_block_with(self.goal)
    }

    pub fn lose(&self) -> bool {
        !self.board.has_empty_block()
    }

    pub fn move_up(&mut self) {
        if self.board.try_to_move_up() {
            self.board.put_new_block();
        }
    }

    pub fn move_down(&mut self) {
        if self.board.try_to_move_down() {
            self.board.put_new_block();
        }
    }

    pub fn move_left(&mut self) {
        if self.board.try_to_move_left() {
            self.board.put_new_block();
        }
    }

    pub fn move_right(&mut self) {
        if self.board.try_to_move_right() {
            self.board.put_new_block();
        }
    }

    pub fn print_board(&self) {
        self.board.print()
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

