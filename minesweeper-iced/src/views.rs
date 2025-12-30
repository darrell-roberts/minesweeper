//! Application views.
mod cell;
mod header;
mod scoreboard;

pub use cell::{CellView, cell_view};
pub use header::Header;
pub use scoreboard::ScoreBoard;

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
