use crate::{
  game::{FlagResult, Game, OpenResult, Position},
  WrappedGame,
};
use minesweeper::model::GameState;
use tauri::State;

#[tauri::command]
pub fn open(position: Position, game: State<WrappedGame>) -> OpenResult {
  let mut g = game.write().unwrap();
  let opened_cells = g.open_cell(position);
  let game_state = *g.board.state();
  let opened_cells = match game_state {
    GameState::Loss | GameState::Win => g.positions(),
    _ => opened_cells,
  };

  OpenResult {
    opened_cells,
    game_state,
    total_mines: g.board.mined(),
  }
}

#[tauri::command]
pub fn flag(position: Position, game: State<WrappedGame>) -> FlagResult {
  let mut g = game.write().unwrap();
  FlagResult {
    position: g.flag_cell(position),
  }
}

#[tauri::command]
pub fn new_game(game: State<WrappedGame>) -> Vec<Position> {
  let new_game = Game::default();
  let positions = new_game
    .board
    .positions()
    .enumerate()
    .map(|(index, (&pos, &cell))| Position { index, pos, cell })
    .collect();
  *game.write().unwrap() = new_game;
  positions
}
