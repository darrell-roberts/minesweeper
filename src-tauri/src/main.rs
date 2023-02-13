#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use app::{
  commands::{flag, new_game, open},
  game::Game,
  TimeEvent, WrappedGame, __cmd__flag, __cmd__new_game, __cmd__open,
};
use minesweeper::model::GameState;
use std::{
  sync::{Arc, RwLock},
  time::Duration,
};
use tauri::Manager;

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
    .invoke_handler(tauri::generate_handler![open, new_game, flag])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
