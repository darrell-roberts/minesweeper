use crate::model::Pos;
use std::{
    num::{NonZeroU8, ParseIntError},
    str::FromStr,
};
use thiserror::Error;

pub mod history;
pub mod model;

/// User command.
#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Open(Pos),
    Flag(Pos),
    Quit,
}

impl FromStr for Command {
    type Err = InvalidCommand;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(' ').collect::<Vec<_>>();
        match parts.as_slice() {
            ["q"] => Ok(Command::Quit),
            &["o", x, y] => parse_coords(x, y).map(Command::Open),
            &["f", x, y] => parse_coords(x, y).map(Command::Flag),
            _ => Err(InvalidCommand::Command(s.to_owned())),
        }
    }
}

/// Parse coordinates provided by user.
fn parse_coords(x: &str, y: &str) -> Result<Pos, InvalidCommand> {
    x.parse::<NonZeroU8>()
        .and_then(|x| y.parse().map(|y| (x, y).into()))
        .map(Ok)?
}

/// Invalid command error.
#[derive(Debug, Error)]
pub enum InvalidCommand {
    #[error("Invalid command: {0}")]
    Command(String),
    #[error("Invalid dimension: {0}")]
    Dimension(#[from] ParseIntError),
    #[error("IO Error: {0}")]
    IO(#[from] std::io::Error),
}
