//! API for handling top 10 wins.
use crate::{format_elapsed, game::Game};
use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use rmp_serde::{encode::write_named, from_read};
use serde::{Deserialize, Serialize};
use std::{
  fs::{create_dir_all, OpenOptions},
  io::{BufWriter, Seek},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Win {
  date: DateTime<Local>,
  duration: u64,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct WinHistory {
  wins: Vec<Win>,
}

#[derive(Serialize, Debug)]
pub struct WinHistoryView {
  wins: Vec<WinView>,
}

#[derive(Serialize, Debug)]
pub struct WinView {
  date: String,
  duration: String,
}

impl From<WinHistory> for WinHistoryView {
  fn from(history: WinHistory) -> Self {
    Self {
      wins: history.wins.into_iter().map(WinView::from).collect(),
    }
  }
}

impl From<Win> for WinView {
  fn from(win: Win) -> Self {
    Self {
      date: format!("{}", win.date.format("%b %e / %G %R")),
      duration: format_elapsed(win.duration),
    }
  }
}

#[cfg(test)]
const SAVE_FILE_PATH: &str = "/tmp/minesweeper/";

#[cfg(not(test))]
const SAVE_FILE_PATH: &str = ".local/share/minesweeper/";

#[cfg(not(test))]
fn get_save_file() -> Result<String> {
  std::env::var("HOME")
    .with_context(|| "Could not lookup $HOME environment variable")
    .map(|home| [&home, "/", SAVE_FILE_PATH, SAVE_FILE].concat())
    .map(Ok)?
}

#[cfg(test)]
fn get_save_file() -> Result<String> {
  Ok([SAVE_FILE_PATH, SAVE_FILE].concat())
}

#[cfg(not(test))]
fn get_full_save_path() -> Result<String> {
  std::env::var("HOME")
    .with_context(|| "Could not lookup up $HOME environment variable")
    .map(|home| [&home, "/", SAVE_FILE_PATH].concat())
    .map(Ok)?
}

#[cfg(test)]
fn get_full_save_path() -> Result<String> {
  Ok(SAVE_FILE_PATH.into())
}

const SAVE_FILE: &str = "stats.bin";

/// Save the win to the win history.
pub fn save_win(game: &Game) -> Result<()> {
  let duration = game
    .start_time
    .map(|st| st.elapsed().as_secs() - game.paused_time)
    .unwrap_or_default();
  persist_win(Win {
    duration,
    date: Local::now(),
  })
}

/// Load win history.
pub fn load_wins() -> Option<WinHistoryView> {
  OpenOptions::new()
    .read(true)
    .open(get_save_file().ok()?)
    .ok()
    .and_then(|stats_file| from_read::<_, WinHistory>(&stats_file).ok())
    .map(WinHistoryView::from)
}

// Save the best 10 results as a MessagePack format.
fn persist_win(win: Win) -> anyhow::Result<()> {
  create_dir_all(get_full_save_path()?)
    .with_context(|| "Could not create folder for stats file")?;
  let mut stats_file = OpenOptions::new()
    .write(true)
    .read(true)
    .create(true)
    .open(get_save_file()?)?;
  let mut history: WinHistory = from_read(&stats_file).unwrap_or_else(|err| {
    eprintln!("Failed to read stats file: {err}. Creating new WinHistory");
    WinHistory::default()
  });
  history.wins.push(win);
  history.wins.sort_by_key(|win| win.duration);
  history.wins = history.wins.into_iter().take(10).collect();
  stats_file.rewind()?;
  let mut writer = BufWriter::new(stats_file);
  write_named(&mut writer, &history)?;
  Ok(())
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_write() {
    let win = Win {
      date: Local::now(),
      duration: 100,
    };

    persist_win(win).unwrap();
  }
}
