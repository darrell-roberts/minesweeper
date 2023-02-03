//! Board implementation for handling game play.
use super::{Board, Cell, CellExpandIter, CellState, GameState, Pos};
use std::{collections::BTreeMap, num::NonZeroU8};

impl Board {
  /// Create a new board with the given columns and rows.
  pub fn new(columns: NonZeroU8, rows: NonZeroU8) -> Self {
    // Generate a cartesian product. Similar to my approach in Haskell.
    let cells = (1..=rows.get())
      .flat_map(|y| (1..=columns.get()).map(move |x| (x, y)))
      .map(|p| (p.try_into().expect("No zero"), Cell::default()))
      .collect::<BTreeMap<_, _>>();
    Board {
      cells,
      columns,
      rows,
      state: GameState::New,
      opened: 0,
    }
  }

  /// Randomly mine the board with a difficulty ratio. Exclude mining the provided position.
  fn mine_board(&mut self, exclude_pos: &Pos) {
    // Set the total amount of mined cells based on
    // difficulty level.
    let total_mined_cells = (f64::from(self.rows.get())
      * f64::from(self.columns.get())
      * 0.10) as usize;

    // Iterator yielding mined positions.
    let mined_positions =
      Pos::random_positions(self.columns.get(), self.rows.get(), *exclude_pos)
        .take(total_mined_cells);

    // Update cell status for mined positions and mined counts.
    for pos in mined_positions {
      if let Some(mined) =
        self
          .cells
          .get_mut(&pos)
          .into_iter()
          .find_map(|c| match &mut c.state {
            CellState::Closed { mined, .. } => (!*mined).then_some(mined),
            _ => None,
          })
      {
        *mined = true;
        for adj in pos.adjacent(self.rows.get(), self.columns.get()) {
          self.cells.entry(adj).and_modify(|c| c.adjacent_mines += 1);
        }
      }
    }
  }

  /// Get the total number of mined cells.
  pub fn total_mines(&self) -> usize {
    self
      .cells
      .values()
      .filter(|&&cell| match cell.state {
        CellState::Closed { mined, .. } => mined,
        CellState::ExposedMine => true,
        _ => false,
      })
      .count()
  }

  /// Return an iterator of all positions that are safe to open and have been opened.
  fn expand(&mut self, pos: Pos) -> impl Iterator<Item = (Pos, Cell)> + '_ {
    CellExpandIter::new(
      pos,
      &mut self.cells,
      self.rows.get(),
      self.columns.get(),
    )
  }

  /// Open a cell and adjacent cells that have no mine counts.
  pub fn open_cell(&mut self, pos: Pos) -> Vec<(Pos, Cell)> {
    if self.state == GameState::New {
      // This is the first move in the game. We will mine the
      // board now and avoid mining the position being opened.
      self.mine_board(&pos);
      self.state = GameState::Active;
    }

    let mut opened_positions = vec![];
    if let Some(c) = self.cells.get_mut(&pos) {
      match c.state {
        CellState::Closed {
          mined: true,
          flagged: false,
        } => {
          self.expose_mines();
          self.state = GameState::Loss;
        }
        CellState::Closed {
          mined: false,
          flagged: false,
        } => {
          c.state = CellState::Open;
          opened_positions.push((pos, *c));
          if c.adjacent_mines == 0 {
            opened_positions.extend(self.expand(pos));
          }
        }
        _ => (),
      }
    }
    self.opened += opened_positions.len();
    if self.is_win() {
      self.state = GameState::Win;
    }
    opened_positions
  }

  /// Expose all mined cells on the board.
  fn expose_mines(&mut self) {
    for c in self.cells.values_mut().filter(|c| c.is_closed_and_mined()) {
      c.state = CellState::ExposedMine
    }
  }

  /// Flag the cell as being potentially mined.
  pub fn flag_cell(&mut self, pos: Pos) -> Option<(Pos, Cell)> {
    self.cells.entry(pos).and_modify(|c| {
      if let CellState::Closed { flagged, .. } = &mut c.state {
        *flagged = !*flagged;
      }
    });
    self.cells.get(&pos).map(|&cell| (pos, cell))
  }

  /// Get the state of the board.
  pub fn state(&self) -> &GameState {
    &self.state
  }

  /// Get the number of board cells.
  pub fn board_size(&self) -> usize {
    self.cells.len()
  }

  /// Evaluate board to see if all non mined cells have been opened.
  fn is_win(&self) -> bool {
    let opened_cells = self
      .cells
      .values()
      .filter(|&&cell| matches!(cell.state, CellState::Open))
      .count();
    let total_open_for_win = self.cells.len() - self.total_mines();
    total_open_for_win == opened_cells || self.all_mines_flagged()
  }

  fn all_mines_flagged(&self) -> bool {
    let flagged_mines = self
      .cells
      .values()
      .filter(|cell| {
        matches!(
          cell.state,
          CellState::Closed {
            flagged: true,
            mined: true
          }
        )
      })
      .count();

    self.total_mines() > 0 && flagged_mines == self.total_mines()
  }
}

#[cfg(test)]
mod test {
  use std::num::NonZeroU8;

  use super::*;

  #[test]
  fn test_board_new() {
    let board_max = NonZeroU8::new(5).unwrap();
    let test_board = Board::new(board_max, board_max);
    dbg!(&test_board);
    assert_eq!(
      test_board.cells.len() as u8,
      board_max.get() * board_max.get()
    );
  }

  #[test]
  fn test_mined_cells() {
    let board_max = NonZeroU8::new(5).unwrap();
    let mut board = Board::new(board_max, board_max);
    board.mine_board(&(1, 1).try_into().unwrap());
    dbg!(&board);
    assert!(board.total_mines() > 0);
  }
}
