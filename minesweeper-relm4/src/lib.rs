use minesweeper::model::Board;
use std::num::NonZeroU8;

mod components;
mod types;

pub use components::app::AppModel;

/// Create a new board 20 x 20.
pub fn board() -> Board {
    Board::new(NonZeroU8::new(20).unwrap(), NonZeroU8::new(20).unwrap())
}

/// Displayable elapsed time.
fn format_elapsed(seconds: u64) -> String {
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
