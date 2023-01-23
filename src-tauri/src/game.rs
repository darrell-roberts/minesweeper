use minesweeper::model::{Board, GameState};
use minesweeper::model::{Cell, Pos};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::num::NonZeroU8;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Position {
    pub index: usize,
    pub pos: Pos,
    pub cell: Cell,
}

#[derive(Debug)]
pub struct Game {
    pub board: Board,
    pub pos_map: HashMap<Pos, usize>,
    pub positions: Vec<Position>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenResult {
    pub opened_cells: Vec<Position>,
    pub game_state: GameState,
}

impl Game {
    pub fn positions(&self) -> Vec<Position> {
        self.board
            .positions()
            .enumerate()
            .map(|(index, (&pos, &cell))| Position { index, pos, cell })
            .collect()
    }

    pub fn open_cell(&mut self, position: Position) -> Vec<Position> {
        self.board
            .open_cell(position.pos)
            .into_iter()
            .flat_map(|(pos, cell)| {
                self.pos_map
                    .get(&pos)
                    .map(|&index| Position { pos, cell, index })
            })
            .collect::<Vec<_>>()
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
        }
    }
}

fn board() -> Board {
    Board::new(NonZeroU8::new(20).unwrap(), NonZeroU8::new(20).unwrap())
}
