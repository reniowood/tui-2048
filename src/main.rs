mod board;
mod game;

use std::io;
use game::Game;

fn main() {
  let mut game = Game::new(256, 4, 4);

  game.print_board();

  while !game.win() && !game.lose() {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            match input.trim() {
                "u" => game.move_up(),
                "d" => game.move_down(),
                "l" => game.move_left(),
                "r" => game.move_right(),
                _ => println!("u, d, l, r are only accepted."),
            }
            game.print_board()
        }
        Err(error) => {
            println!("error: {}", error);
            break;
        }
    }
  }

  if game.win() {
      println!("You win");
  } else if game.lose() {
      println!("You lose");
  }
}
