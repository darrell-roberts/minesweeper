use anyhow::{Context, Result};
use app_dirs2::{get_app_root, AppDataType, AppInfo};
use chrono::{DateTime, Local};
use rmp_serde::{encode::write_named, from_read};
use serde::{Deserialize, Serialize};
use std::{
    fs::{create_dir_all, OpenOptions},
    io::{BufWriter, Seek},
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

pub const APP_INFO: AppInfo = AppInfo {
    name: "Minesweeper",
    author: "Somebody",
};

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
    let data_file = get_app_root(AppDataType::UserData, &APP_INFO)
        .context("Could not get app root")
        .ok()?
        .join(SAVE_FILE);

    OpenOptions::new()
        .read(true)
        .open(data_file)
        .ok()
        .and_then(|stats_file| from_read(&stats_file).ok())
}

// Save the best 10 results as a MessagePack format.
fn persist_win(win: Win) -> anyhow::Result<()> {
    let data_path =
        get_app_root(AppDataType::UserData, &APP_INFO).context("Could not get app root")?;

    if !data_path.exists() {
        create_dir_all(&data_path).context("Failed to create app root")?;
    }

    let mut stats_file = OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .truncate(false)
        .open(data_path.join(SAVE_FILE))?;

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
