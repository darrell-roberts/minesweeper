#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use game::{FlagResult, Game, OpenResult, Position};
use minesweeper::model::GameState;
use serde::Serialize;
use std::{
  sync::{Arc, RwLock},
  time::Duration,
};
use tauri::{Manager, State};

mod game;

type WrappedGame = Arc<RwLock<Game>>;

fn get_elapased(game: &Game) -> String {
  let seconds = game.start_time.elapsed().as_secs();

  match seconds {
    0..=59 => format!("{} seconds", seconds),
    60..=3599 => format!(
      "{} minute(s) {} seconds",
      seconds.div_euclid(60),
      seconds.rem_euclid(60)
    ),
    3600.. => format!("{} hours", seconds.div_euclid(3600)),
  }
}

#[tauri::command]
fn open(position: Position, game: State<WrappedGame>) -> OpenResult {
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
fn flag(position: Position, game: State<WrappedGame>) -> FlagResult {
  let mut g = game.write().unwrap();
  FlagResult {
    position: g.flag_cell(position),
  }
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
  *game.write().unwrap() = new_game;
  positions
}

#[derive(Serialize, Clone)]
struct TimeEvent {
  duration: String,
}

fn main() {
  let game: WrappedGame = Arc::new(RwLock::new(Game::default()));
  tauri::Builder::default()
    .manage(game.clone())
    .setup(move |app| {
      let main_window = app.get_window("main").unwrap();
      let game_copy = game.clone();
      std::thread::spawn(move || loop {
        let (state, duration) = {
          game_copy
            .read()
            .map(|g| (*g.board.state(), get_elapased(&*g)))
            .unwrap()
        };
        if matches!(state, GameState::Active | GameState::New) {
          main_window
            .emit("time-event", TimeEvent { duration })
            .unwrap();
        }
        std::thread::sleep(Duration::from_secs(1));
      });
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![open, new_game, flag,])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
