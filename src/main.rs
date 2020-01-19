mod board;
mod event;
mod game;

use event::{Event, Events};
use game::Game;
use std::io;
use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Paragraph, Text, Widget};
use tui::Terminal;

fn main() -> Result<(), failure::Error> {
    let logo = r"
  ___   ___  _  _   ___  
 |__ \ / _ \| || | / _ \ 
    ) | | | | || || (_) |
   / /| | | |__   _> _ < 
  / /_| |_| |  | || (_) |
 |____|\___/   |_| \___/ ";
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = Events::new();

    // Game initialization
    let mut game = Game::new(2048, 4, 4);

    loop {
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(0)
                .constraints(
                    [
                        Constraint::Percentage(30),
                        Constraint::Percentage(40),
                        Constraint::Percentage(30),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            // game board
            {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(0)
                    .constraints(
                        [
                            Constraint::Percentage(25),
                            Constraint::Percentage(60),
                            Constraint::Percentage(15),
                        ]
                        .as_ref(),
                    )
                    .split(chunks[1]);

                // header
                {
                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(
                            [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                        )
                        .split(chunks[0]);

                    let chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Percentage(70), Constraint::Percentage(30)].as_ref(),
                        )
                        .split(chunks[1]);

                    // title
                    {
                        Paragraph::new([Text::raw(logo)].iter())
                            .block(Block::default())
                            .alignment(Alignment::Left)
                            .render(&mut f, chunks[0]);
                    }

                    // status
                    {
                        let message;
                        let score = format!("score: {}", game.score);

                        if game.win() {
                            message = "You win!";
                        } else if game.lose() {
                            message = "You lose!";
                        } else {
                            message = &score;
                        }

                        let block = Block::default().title("status").borders(Borders::ALL);

                        Paragraph::new([Text::raw(message)].iter())
                            .block(block)
                            .alignment(Alignment::Center)
                            .render(&mut f, chunks[1]);
                    }
                }

                Block::default().render(&mut f, chunks[1]);

                {
                    let constraints = (0..game.height)
                        .map(|_i| Constraint::Percentage(100 / (game.height as u16)))
                        .collect::<Vec<Constraint>>();
                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(constraints.as_ref())
                        .split(chunks[1]);
                    for i in 0..game.height {
                        Block::default().render(&mut f, chunks[i]);
                        {
                            let constraints = (0..game.width)
                                .map(|_i| Constraint::Percentage(100 / (game.width as u16)))
                                .collect::<Vec<Constraint>>();
                            let chunks = Layout::default()
                                .direction(Direction::Horizontal)
                                .constraints(constraints.as_ref())
                                .split(chunks[i]);
                            let block = Block::default().borders(Borders::ALL);

                            for j in 0..game.width {
                                Paragraph::new(
                                    [Text::styled(
                                        game.board.blocks[i][j]
                                            .map(|x| format!("{}", x))
                                            .unwrap_or("".to_string()),
                                        Style::default().fg(Color::Rgb(
                                            255,
                                            game.board.blocks[i][j]
                                                .map(|x| 128 + (128 / x) as u8)
                                                .unwrap_or(0),
                                            0,
                                        )),
                                    )]
                                    .iter(),
                                )
                                .block(block.clone())
                                .alignment(Alignment::Center)
                                .render(&mut f, chunks[j]);
                            }
                        }
                    }
                }
            }
        })?;

        match events.next()? {
            Event::Input(key) => match key {
                Key::Char('q') => break,
                _ => {
                    if !game.win() && !game.lose() {
                        match key {
                            Key::Char('h') => game.move_left(),
                            Key::Char('j') => game.move_down(),
                            Key::Char('k') => game.move_up(),
                            Key::Char('l') => game.move_right(),
                            _ => {}
                        }
                    }
                }
            },
            _ => {}
        };
    }

    Ok(())
}
