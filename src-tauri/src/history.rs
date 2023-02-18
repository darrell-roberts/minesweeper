use crate::{format_elapsed, game::Game};
use anyhow::Result;
use chrono::{DateTime, Local};
use rmp_serde::{encode::write_named, from_read};
use serde::{Deserialize, Serialize};
use std::{
  fs::{create_dir_all, OpenOptions},
  io::{BufWriter, Seek},
  time::Duration,
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
      date: format!("{}", win.date.format("%v %r")),
      duration: format_elapsed(Duration::from_secs(win.duration)),
    }
  }
}

#[cfg(test)]
const SAVE_FILE_PATH: &str = "/tmp/minesweeper/";

#[cfg(not(test))]
const SAVE_FILE_PATH: &str = ".local/share/minesweeper/";

#[cfg(not(test))]
const HOME: &str = env!("HOME");

const SAVE_FILE: &str = "stats.bin";

pub fn save_win(game: &Game) -> Result<()> {
  let duration = game.start_time.elapsed().as_secs();
  let date = Local::now();
  let win = Win { date, duration };

  persist_win(win)?;

  Ok(())
}

#[cfg(not(test))]
fn get_full_save_file_path() -> String {
  [HOME, "/", SAVE_FILE_PATH, SAVE_FILE].concat()
}

#[cfg(test)]
fn get_full_save_file_path() -> String {
  [SAVE_FILE_PATH, SAVE_FILE].concat()
}

pub fn load_wins() -> Option<WinHistoryView> {
  OpenOptions::new()
    .read(true)
    .open(get_full_save_file_path())
    .ok()
    .and_then(|stats_file| from_read::<_, WinHistory>(&stats_file).ok())
    .map(WinHistoryView::from)
}

fn persist_win(win: Win) -> Result<(), anyhow::Error> {
  create_dir_all(SAVE_FILE_PATH)?;
  let mut stats_file = OpenOptions::new()
    .write(true)
    .read(true)
    .create(true)
    .open(get_full_save_file_path())?;
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
