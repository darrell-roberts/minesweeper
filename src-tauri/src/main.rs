#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use minesweeper::model::Board;
use minesweeper::model::{Cell, Pos};
use serde::Serialize;
use std::num::NonZeroU8;
use std::sync::{Arc, Mutex};
use tauri::State;

#[derive(Debug, Copy, Clone, Serialize)]
pub struct Position {
  pub index: usize,
  pub pos: Pos,
  pub cell: Cell,
}

#[derive(Debug)]
struct Game {
  board: Arc<Mutex<Board>>,
}

fn board() -> Board {
  Board::new(NonZeroU8::new(20).unwrap(), NonZeroU8::new(20).unwrap())
}

#[tauri::command]
fn get_positions(game: State<Game>) -> Vec<Position> {
  game
    .board
    .lock()
    .unwrap()
    .positions()
    .enumerate()
    .map(|(index, (&pos, &cell))| Position { index, pos, cell })
    .collect()
}

#[tauri::command]
fn open(position: Position, game: State<Game>) {
  game.board.lock().unwrap().open_cell(position.pos);
}

fn main() {
  tauri::Builder::default()
    .manage(Game {
      board: Arc::new(Mutex::new(board())),
    })
    .invoke_handler(tauri::generate_handler![get_positions])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
