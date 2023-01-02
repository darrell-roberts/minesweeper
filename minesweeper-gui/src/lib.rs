use crate::widgets::AppWidgets;
use minesweeper::model::{Board, GameState, Pos};
use relm4::{factory::FactoryVec, AppUpdate, Model, RelmComponent, Sender};
use status_dialog::{StatusDialogModel, StatusMsg};
use std::{collections::HashMap, num::NonZeroU8};
use types::Position;

mod status_dialog;
mod types;
mod widgets;

/// Application state.
pub struct AppModel {
  /// Game board and API
  board: Board,
  /// View model for board.
  positions: FactoryVec<Position>,
  /// Map Pos items to index in [FactoryVec].
  pos_map: HashMap<Pos, usize>,
}

fn board() -> Board {
  Board::new(NonZeroU8::new(20).unwrap(), NonZeroU8::new(20).unwrap())
}

impl Default for AppModel {
  fn default() -> Self {
    let board = board();
    let positions = FactoryVec::from_vec(
      board
        .positions()
        .enumerate()
        .map(|(index, (&pos, &cell))| Position { index, pos, cell })
        .collect(),
    );

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

impl AppModel {
  /// Sync up the view model with the game board.
  fn update_all_positions(&mut self) {
    self.positions.clear();
    for (index, (&pos, &cell)) in self.board.positions().enumerate() {
      self.positions.push(Position { index, pos, cell });
    }
  }

  /// Replace View positions cells with updated board cell.
  fn update_positions(&mut self, positions: &[Position]) {
    for p in positions {
      if let Some(pos) = self.positions.get_mut(p.index) {
        *pos = *p;
      }
    }
  }
}

/// User actions
pub enum AppMsg {
  /// Open a position on the board.
  Open(Pos),
  /// Flag a position on the board.
  Flag(Position),
  /// Start a new game, resetting the board.
  Start,
}

impl Model for AppModel {
  type Components = AppComponents;
  type Msg = AppMsg;
  type Widgets = AppWidgets;
}

impl AppUpdate for AppModel {
  fn update(
    &mut self,
    msg: Self::Msg,
    components: &Self::Components,
    _sender: relm4::Sender<Self::Msg>,
  ) -> bool {
    match msg {
      AppMsg::Open(p) => {
        let opened = self.board.open_cell(p);

        match *self.board.state() {
          s @ GameState::Loss | s @ GameState::Win => {
            self.update_all_positions();
            components
              .status_dialog
              .send(StatusMsg::Open(if s == GameState::Loss {
                String::from("You Lose!")
              } else {
                String::from("You Win!")
              }))
              .unwrap_or_else(|e| {
                eprintln!("Failed to send message {e}");
              });
          }
          _ => {
            let matched_pos = opened
              .into_iter()
              .flat_map(|(pos, cell)| {
                self.pos_map.get(&pos).map(|&index| Position {
                  pos,
                  cell,
                  index,
                })
              })
              .collect::<Vec<_>>();
            self.update_positions(&matched_pos);
          }
        }
      }
      AppMsg::Flag(p) => {
        if let Some(position) =
          self.board.flag_cell(p.pos).and_then(|(pos, cell)| {
            self
              .pos_map
              .get(&pos)
              .map(|&index| Position { pos, cell, index })
          })
        {
          self.update_positions(&[position]);
        }
      }
      AppMsg::Start => {
        self.board = board();
        self.update_all_positions();
      }
    }

    true
  }
}

#[derive(relm4::Components)]
pub struct AppComponents {
  status_dialog: RelmComponent<StatusDialogModel, AppModel>,
}
