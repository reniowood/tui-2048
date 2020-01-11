struct Board {
    width: u32,
    height: u32,
    blocks: Vec<Vec<Option<u32>>>
}

impl Board {
    fn new(width: u32, height: u32) -> Board {
        Board {
            width: width, 
            height: height,
            blocks: vec![vec![None; width as usize]; height as usize],
        }
    }

    fn move_blocks(&self, mv: &Move) {
    }
}

struct Game {
    goal: u32,
    board: Board,
}

impl Game {
    fn new(goal: u32, width: u32, height: u32) -> Game {
        Game {
            goal: goal,
            board: Board::new(width, height),
        }
    }

    fn win(&self) -> bool {
        false
    }

    fn lose(&self) -> bool {
        false
    }

    fn play(&self, mv: &Move) {
    }
}

enum Move {
    Up, Down, Left, Right,
}

fn main() {
    println!("Hello, world!");
}
