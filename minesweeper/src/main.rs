use clap::Parser;
use minesweeper::{
  model::{Board, GameState},
  Command, InvalidCommand,
};
use std::{
  io::{stdin, stdout, Write},
  num::NonZeroU8,
};

/// Command line arguments.
#[derive(Parser)]
pub struct ProgramArgs {
  #[clap(short, help = "Number of rows", action, default_value = "10")]
  pub rows: NonZeroU8,
  #[clap(short, help = "Number of columns", action, default_value = "10")]
  pub columns: NonZeroU8,
}

/// Parse user input.
fn parse_command() -> Result<Command, InvalidCommand> {
  let mut input = String::new();
  stdin().read_line(&mut input)?;
  input.trim().parse()
}

/// Main game loop. Draws the board and takes user input
/// until win/loss or quit.
fn game_loop(mut board: Board) {
  loop {
    println!("{board}");

    match board.state() {
      GameState::Loss => {
        println!("You Lose!");
        break;
      }
      GameState::Win => {
        println!("You Win!");
        break;
      }
      GameState::Active | GameState::New => {
        print!("(o, f, q): ");
        stdout().flush().unwrap();
        match parse_command() {
          Ok(Command::Quit) => break,
          Ok(Command::Open(p)) => {
            board.open_cell(p);
          }
          Ok(Command::Flag(p)) => {
            board.flag_cell(p);
          }
          Err(e) => {
            eprintln!("Invalid command: {e}");
          }
        }
      }
    }
  }
}

/// Parse command line arguments and start game.
fn main() {
  let ProgramArgs { rows, columns } = ProgramArgs::parse();
  game_loop(Board::new(columns, rows));
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn test_parse() {
    let open = "o 1 1".parse::<Command>().unwrap();
    assert_eq!(open, Command::Open((1, 1).try_into().unwrap()));
    let flag = "f 1 1".parse::<Command>().unwrap();
    assert_eq!(flag, Command::Flag((1, 1).try_into().unwrap()));
    let quit = "q".parse::<Command>().unwrap();
    assert_eq!(quit, Command::Quit);
    let invalid = "abc".parse::<Command>();
    assert!(matches!(invalid, Err(InvalidCommand::Command(s)) if s == "abc"));
    let overflow = "o 260 2".parse::<Command>();
    dbg!(&overflow);
    assert!(matches!(overflow, Err(InvalidCommand::Dimension(_))));
  }
}
