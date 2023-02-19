//! Game types and trait implementations.
mod game;

use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
  cmp::Ordering,
  collections::{BTreeMap, HashSet},
  fmt::{Display, Formatter},
  num::{NonZeroU8, TryFromIntError},
};

/// Board cell.
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cell {
  pub state: CellState,
  pub adjacent_mines: u8,
}

impl Cell {
  fn is_closed_and_mined(&self) -> bool {
    matches!(self.state, CellState::Closed { mined: true, .. })
  }
}

/// Cell position on the board.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Pos {
  pub x: NonZeroU8,
  pub y: NonZeroU8,
}

impl PartialOrd for Pos {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.y.cmp(&other.y).then(self.x.cmp(&other.x)))
  }
}

impl Ord for Pos {
  fn cmp(&self, other: &Self) -> Ordering {
    self.y.cmp(&other.y).then(self.x.cmp(&other.x))
  }
}

impl TryFrom<(u8, u8)> for Pos {
  type Error = TryFromIntError;
  fn try_from((x, y): (u8, u8)) -> Result<Self, Self::Error> {
    let pos = Self {
      x: x.try_into()?,
      y: y.try_into()?,
    };
    Ok(pos)
  }
}

impl From<(NonZeroU8, NonZeroU8)> for Pos {
  fn from((x, y): (NonZeroU8, NonZeroU8)) -> Self {
    Self { x, y }
  }
}

/// State of the cell.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum CellState {
  Open,
  Closed { flagged: bool, mined: bool },
  ExposedMine,
}

impl Default for CellState {
  fn default() -> Self {
    CellState::Closed {
      flagged: false,
      mined: false,
    }
  }
}

impl Display for Cell {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let num = self.adjacent_mines.to_string();
    write!(
      f,
      "{}",
      match self.state {
        CellState::Open =>
          if self.adjacent_mines > 0 {
            num.chars().next().expect("1 - 8 value")
          } else {
            ' '
          },
        CellState::Closed { flagged, .. } =>
          if flagged {
            'F'
          } else {
            '.'
          },
        CellState::ExposedMine => 'X',
      }
    )
  }
}

/// State of the game.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize)]
pub enum GameState {
  New,
  Active,
  Loss,
  Win,
}

/// Game board.
#[derive(Debug)]
pub struct Board {
  cells: BTreeMap<Pos, Cell>,
  columns: NonZeroU8,
  rows: NonZeroU8,
  state: GameState,
  opened: usize,
  flagged: usize,
  mined: usize,
}

impl Board {
  pub fn positions(&self) -> impl Iterator<Item = (&Pos, &Cell)> {
    self.cells.iter()
  }

  pub fn get_pos(&self, pos: &Pos) -> Option<&Cell> {
    self.cells.get(pos)
  }

  pub fn total_rows(&self) -> NonZeroU8 {
    self.rows
  }

  pub fn total_columns(&self) -> NonZeroU8 {
    self.columns
  }

  pub fn opened(&self) -> usize {
    self.opened
  }

  pub fn flagged(&self) -> usize {
    self.flagged
  }

  pub fn mined(&self) -> usize {
    self.mined
  }
}

impl Display for Board {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "board: {}, mines: {}", self.board_size(), self.mined)?;
    write!(f, "   ")?;
    for c in 1..=self.columns.get() {
      write!(f, "{c:<3}")?;
    }
    for (pos, cell) in self.cells.iter() {
      if pos.x.get() == 1 {
        write!(f, "\n{:<2} {cell}  ", pos.y)?;
      } else {
        write!(f, "{cell:<3}  ")?;
      }
    }
    writeln!(f)
  }
}

struct CellExpandIter<'a> {
  start: Option<Pos>,
  board: &'a mut BTreeMap<Pos, Cell>,
  visited: HashSet<Pos>,
  adjacent: Vec<Pos>,
  total_rows: u8,
  total_columns: u8,
}

impl<'a> CellExpandIter<'a> {
  pub fn new(
    pos: Pos,
    board: &'a mut BTreeMap<Pos, Cell>,
    total_rows: u8,
    total_columns: u8,
  ) -> Self {
    CellExpandIter {
      start: Some(pos),
      board,
      visited: HashSet::new(),
      adjacent: vec![],
      total_rows,
      total_columns,
    }
  }
}

/// An iterator that returns Positions that were opened from the starting
/// position.
impl<'a> Iterator for CellExpandIter<'a> {
  type Item = (Pos, Cell);

  fn next(&mut self) -> Option<Self::Item> {
    // Start of iteration.
    if let Some(p) = self.start.take() {
      self.adjacent = p.adjacent(self.total_rows, self.total_columns).collect();
    }

    while !self.adjacent.is_empty() {
      while let Some(p) = self.adjacent.pop() {
        if self.visited.contains(&p) {
          continue;
        }

        self.visited.insert(p);
        if let Some(c) = self.board.get_mut(&p).filter(|c| {
          matches!(
            c.state,
            CellState::Closed {
              flagged: false,
              mined: false
            }
          )
        }) {
          if c.adjacent_mines == 0 {
            self
              .adjacent
              .extend(p.adjacent(self.total_rows, self.total_columns));
          }
          c.state = CellState::Open;
          return Some((p, *c));
        }
      }
    }
    None
  }
}

/// A Random Position iterator that yields unique positions within
/// range.
struct RandomPosIter {
  used_positions: HashSet<Pos>,
  columns: u8,
  rows: u8,
  rng: ThreadRng,
}

impl RandomPosIter {
  fn new(columns: u8, rows: u8, exclude: Vec<Pos>) -> Self {
    Self {
      used_positions: HashSet::from_iter(exclude.into_iter()),
      rng: Default::default(),
      rows,
      columns,
    }
  }
}

impl Iterator for RandomPosIter {
  type Item = Pos;
  fn next(&mut self) -> Option<Self::Item> {
    loop {
      let x = self.rng.gen_range(1..self.columns);
      let y = self.rng.gen_range(1..self.rows);

      let pos = Pos::try_from((x, y)).ok()?;

      if self.used_positions.len()
        == usize::from(self.rows) * usize::from(self.columns)
      {
        return None;
      }

      if !self.used_positions.contains(&pos) {
        self.used_positions.insert(pos);
        return Some(Pos::try_from((x, y)).expect("No zero"));
      }
    }
  }
}

impl Pos {
  /// Yields unique random positions within range and exclusion.
  fn random_positions(
    columns: u8,
    rows: u8,
    exclude: Vec<Pos>,
  ) -> impl Iterator<Item = Pos> {
    RandomPosIter::new(columns, rows, exclude)
  }

  /// Yields adjacent positions within bounds.
  fn adjacent(
    &self,
    max_rows: u8,
    max_columns: u8,
  ) -> impl Iterator<Item = Pos> + '_ {
    let &Pos { x, y } = &self;
    let edges = |n| [n - 1, n, n + 1].into_iter();

    edges(x.get())
      .flat_map(move |x1| edges(y.get()).map(move |y1| (x1, y1)))
      .filter(move |&(x1, y1)| {
        x1 > 0 && y1 > 0 && (x1, y1) != (x.get(), y.get())
      })
      .filter(move |&(x, y)| x <= max_rows && y <= max_columns)
      .map(Pos::try_from)
      .flat_map(|r| r.ok())
  }
}

#[cfg(test)]
mod test {
  use super::Pos;
  #[test]
  fn test_adjacent() {
    let adjacent = Pos::try_from((1, 1))
      .unwrap()
      .adjacent(10, 10)
      .collect::<Vec<_>>();
    dbg!(&adjacent);
    assert_eq!(adjacent.len(), 3);
    let adjacent = Pos::try_from((3, 3))
      .unwrap()
      .adjacent(10, 10)
      .collect::<Vec<_>>();
    dbg!(&adjacent);
    assert_eq!(adjacent.len(), 8);
    let adjacent = Pos::try_from((2, 1))
      .unwrap()
      .adjacent(10, 10)
      .collect::<Vec<_>>();
    dbg!(&adjacent);
  }
}
