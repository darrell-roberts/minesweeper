//! API commands for the tauri client. These API's expose
//! game functions and state management.
use crate::{
    game::{FlagResult, Game, OpenResult, Position},
    history::WinHistoryView,
    AppGame,
};
use minesweeper::{
    history::{load_wins, save_win},
    model::GameState,
};
use std::time::Instant;
use tauri::State;

/// Open a cell.
#[tauri::command]
pub fn open(position: Position, game: State<AppGame>) -> OpenResult {
    let mut g = game.write().unwrap();
    // The first move will start the clock.
    if matches!(g.board.state(), GameState::New) {
        g.start_time = Some(Instant::now());
    }
    let opened_cells = g.open_cell(position);
    let game_state = *g.board.state();

    // If the opened position results in a win or loss then
    // we'll return all positions on the board otherwise
    // just the opened cells.
    let opened_cells = match game_state {
        GameState::Loss | GameState::Win => g.positions(),
        _ => opened_cells,
    };

    // Save the win history.
    if matches!(game_state, GameState::Win) {
        let duration = g
            .start_time
            .map(|st| st.elapsed().as_secs() - g.paused_time)
            .unwrap_or_default();
        if let Err(err) = save_win(duration) {
            eprintln!("Failed to save game state {err}");
        }
    }

    OpenResult {
        opened_cells,
        game_state,
        total_mines: g.board.mined(),
    }
}

/// Flag a cell.
#[tauri::command]
pub fn flag(position: Position, game: State<AppGame>) -> FlagResult {
    let mut g = game.write().unwrap();
    FlagResult {
        position: g.flag_cell(position),
    }
}

/// Start a new game.
#[tauri::command]
pub fn new_game(game: State<AppGame>) -> Vec<Position> {
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

/// Get the top 10 wins.
#[tauri::command]
pub fn get_win_history(game: State<AppGame>) -> Option<WinHistoryView> {
    {
        let mut g = game.write().unwrap();
        g.paused = Some(Instant::now());
    }
    load_wins().map(Into::into)
}

/// Resume a game that is paused.
#[tauri::command]
pub fn resume(game: State<AppGame>) {
    let mut g = game.write().unwrap();
    if let Some(p) = g.paused.take() {
        g.paused_time += p.elapsed().as_secs();
    }
}

#[tauri::command]
pub fn platform() -> &'static str {
    if cfg!(target_os = "macos") {
        "mac"
    } else {
        "other"
    }
}
