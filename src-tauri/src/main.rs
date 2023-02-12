#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use game::{Game, OpenResult, Position};
use minesweeper::model::GameState;
use std::sync::{Arc, Mutex};
use tauri::State;

mod game;

type WrappedGame = Arc<Mutex<Game>>;

#[tauri::command]
fn open(position: Position, game: State<WrappedGame>) -> OpenResult {
  let mut g = game.lock().unwrap();
  let opened_cells = g.open_cell(position);
  let game_state = *g.board.state();
  let opened_cells = match game_state {
    GameState::Loss | GameState::Win => g.positions(),
    _ => opened_cells,
  };
  let seconds = g.start_time.elapsed().as_secs();

  let duration_str = match seconds {
    0..=59 => format!("{} seconds", seconds),
    60..=3599 => format!(
      "{} minute(s) {} seconds",
      seconds.div_euclid(60),
      seconds.rem_euclid(60)
    ),
    3600.. => format!("{} hours", seconds.div_euclid(3600)),
  };

  OpenResult {
    opened_cells,
    game_state,
    total_mines: g.board.mined(),
    duration: duration_str,
  }
}

#[tauri::command]
fn flag(position: Position, game: State<WrappedGame>) -> Option<Position> {
  game.lock().unwrap().flag_cell(position)
}

#[tauri::command]
fn new_game(game: State<WrappedGame>) -> Vec<Position> {
  let new_game = Game::default();
  let positions = new_game
    .board
    .positions()
    .enumerate()
    .map(|(index, (&pos, &cell))| Position { index, pos, cell })
    .collect();
  *game.lock().unwrap() = new_game;
  positions
}

fn main() {
  let game: WrappedGame = Arc::new(Mutex::new(Game::default()));
  tauri::Builder::default()
    .manage(game)
    .invoke_handler(tauri::generate_handler![open, new_game, flag])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
