struct Board {
    width: u32,
    height: u32,
    values: Vec<Vec<Option<u32>>>
}

struct Game {
    goal: u32,
    board: Board,
}

fn main() {
    println!("Hello, world!");
}
