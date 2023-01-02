use minesweeper::model::{Cell, Pos};

#[derive(Debug, Copy, Clone)]
pub struct Position {
  pub index: usize,
  pub pos: Pos,
  pub cell: Cell,
}
