#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use app::{
  commands::{flag, get_win_history, new_game, open},
  game::Game,
  AppGame, TimeEvent, __cmd__flag, __cmd__get_win_history, __cmd__new_game,
  __cmd__open, format_elapsed,
};
use minesweeper::model::GameState;
use std::{
  sync::{Arc, RwLock},
  time::Duration,
};
use tauri::Manager;

fn main() {
  let game: AppGame = Arc::new(RwLock::new(Game::default()));
  tauri::Builder::default()
    .manage(game.clone())
    .setup(move |app| {
      let main_window = app.get_window("main").unwrap();
      std::thread::spawn(move || loop {
        let (state, duration) = {
          game
            .read()
            .map(|g| (*g.board.state(), format_elapsed(g.start_time.elapsed())))
            .unwrap()
        };
        if matches!(state, GameState::Active | GameState::New) {
          main_window
            .emit("time-event", TimeEvent { duration })
            .unwrap_or_else(|e| eprintln!("Failed to emit time event {e}"));
        }
        std::thread::sleep(Duration::from_secs(1));
      });
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      open,
      new_game,
      flag,
      get_win_history
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
