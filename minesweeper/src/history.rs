use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use rmp_serde::{encode::write_named, from_read};
use serde::{Deserialize, Serialize};
use std::{
    fs::{create_dir_all, File, OpenOptions},
    io::BufWriter,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Win {
    pub date: DateTime<Local>,
    pub duration: u64,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct WinHistory {
    pub wins: Vec<Win>,
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
pub fn save_win(duration: u64) -> Result<()> {
    persist_win(Win {
        duration,
        date: Local::now(),
    })
}

/// Load win history.
pub fn load_wins() -> Option<WinHistory> {
    OpenOptions::new()
        .read(true)
        .open(get_save_file().ok()?)
        .ok()
        .and_then(|stats_file| from_read(&stats_file).ok())
}

// Save the best 10 results as a MessagePack format.
fn persist_win(win: Win) -> anyhow::Result<()> {
    create_dir_all(get_full_save_path()?)
        .with_context(|| "Could not create folder for stats file")?;

    let save_file = get_save_file()?;
    let mut history: WinHistory = from_read(File::open(&save_file)?).unwrap_or_else(|err| {
        eprintln!("Failed to read stats file: {err}. Creating new WinHistory");
        WinHistory::default()
    });
    history.wins.push(win);
    history.wins.sort_by_key(|win| win.duration);
    history.wins = history.wins.into_iter().take(10).collect();
    let stats_file = File::create(&save_file)?;
    let mut writer = BufWriter::new(stats_file);
    write_named(&mut writer, &history)?;
    Ok(())
}
