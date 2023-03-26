//! Wrapper for the minesweeper game used with a Tauri user
//! interface.
use minesweeper::model::{Board, Cell, GameState, Pos};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, num::NonZeroU8, time::Instant};

/// Cell position with an index.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Position {
  pub index: usize,
  pub pos: Pos,
  pub cell: Cell,
}

/// Game and state.
#[derive(Debug)]
pub struct Game {
  pub board: Board,
  pub pos_map: HashMap<Pos, usize>,
  pub positions: Vec<Position>,
  pub start_time: Option<Instant>,
  pub paused_time: u64,
  pub paused: Option<Instant>,
}

/// Command response for opening a cell.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenResult {
  pub opened_cells: Vec<Position>,
  pub game_state: GameState,
  pub total_mines: usize,
}

/// Command response for flagging a cell.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlagResult {
  pub position: Option<Position>,
}

impl Game {
  /// Get positions with indices.
  pub fn positions(&self) -> Vec<Position> {
    self
      .board
      .positions()
      .enumerate()
      .map(|(index, (&pos, &cell))| Position { index, pos, cell })
      .collect()
  }

  /// Open a cell on the board.
  pub fn open_cell(&mut self, position: Position) -> Vec<Position> {
    self
      .board
      .open_cell(position.pos)
      .into_iter()
      .flat_map(|(pos, cell)| {
        self
          .pos_map
          .get(&pos)
          .map(|&index| Position { pos, cell, index })
      })
      .collect::<Vec<_>>()
  }

  /// Flag a cell on the board.
  pub fn flag_cell(&mut self, position: Position) -> Option<Position> {
    self
      .board
      .flag_cell(position.pos)
      .map(|(pos, cell)| Position {
        index: position.index,
        pos,
        cell,
      })
  }
}

impl Default for Game {
  fn default() -> Self {
    let board = board();
    let positions = board
      .positions()
      .enumerate()
      .map(|(index, (&pos, &cell))| Position { index, pos, cell })
      .collect::<Vec<_>>();

    let pos_map = positions
      .iter()
      .map(|Position { index, pos, .. }| (*pos, *index))
      .collect::<HashMap<_, _>>();

    Self {
      board,
      positions,
      pos_map,
      start_time: None,
      paused_time: 0,
      paused: None,
    }
  }
}

/// Create a new un-mined 20 x 20 board.
fn board() -> Board {
  Board::new(NonZeroU8::new(20).unwrap(), NonZeroU8::new(20).unwrap())
}
