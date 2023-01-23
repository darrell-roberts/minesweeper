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
fn get_positions(game: State<WrappedGame>) -> Vec<Position> {
    game.lock().unwrap().positions()
}

#[tauri::command]
fn open(position: Position, game: State<WrappedGame>) -> OpenResult {
    let mut g = game.lock().unwrap();
    let opened_cells = g.open_cell(position);
    let game_state = *g.board.state();
    let opened_cells = match game_state {
        GameState::Loss | GameState::Win => g.positions(),
        _ => opened_cells,
    };
    OpenResult {
        opened_cells,
        game_state,
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
    *game.lock().unwrap() = new_game;
    positions
}

fn main() {
    let game: WrappedGame = Arc::new(Mutex::new(Game::default()));
    tauri::Builder::default()
        .manage(game)
        .invoke_handler(tauri::generate_handler![get_positions, open, new_game])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
