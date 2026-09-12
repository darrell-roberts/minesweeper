#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use commands::*;
use game::Game;
use minesweeper::model::GameState;
use serde::Serialize;
use std::{
    sync::{Arc, RwLock},
    time::Duration,
};
use tauri::{Emitter, Manager};

pub mod commands;
pub mod game;
pub mod history;

/// Payload for the time event.
#[derive(Serialize, Clone)]
pub struct TimeEvent {
    /// Formatted game time duration.
    pub duration: String,
}

pub type AppGame = Arc<RwLock<Game>>;

/// Displayable elapsed time.
pub fn format_elapsed(seconds: u64) -> String {
    match seconds {
        0..=59 => format!("{seconds} seconds"),
        60..=3599 => format!(
            "{} minute(s) {} seconds",
            seconds.div_euclid(60),
            seconds.rem_euclid(60)
        ),
        3600.. => format!("{} hours", seconds.div_euclid(3600)),
    }
}

fn game_time_elapsed(game: &Arc<RwLock<Game>>) -> Option<String> {
    let guard = game.read().ok()?;

    if guard.paused.is_some() || !matches!(guard.board.state(), GameState::Active) {
        return None;
    }

    guard
        .start_time?
        .elapsed()
        .as_secs()
        .checked_sub(guard.paused_time)
        .map(format_elapsed)
}

fn main() {
    let game: AppGame = Arc::new(RwLock::new(Game::default()));
    tauri::Builder::default()
        .manage(game.clone())
        .setup(move |app| {
            let main_window = app.get_webview_window("main").unwrap();
            std::thread::spawn(move || loop {
                if let Some(duration) = game_time_elapsed(&game) {
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
            get_win_history,
            resume,
            platform
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
