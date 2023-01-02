//! API for handling top 10 wins.
use crate::format_elapsed;
use minesweeper::history::{Win, WinHistory};
use serde::Serialize;

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
