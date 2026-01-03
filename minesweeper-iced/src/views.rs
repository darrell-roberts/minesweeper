//! Application views.
mod cell;
mod header;
mod scoreboard;

pub use cell::{CellView, cell_view};
pub use header::Header;
use iced::{Shadow, Theme, widget::button};
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

/// Create a button shadow. Active when not pressed.
pub fn mk_button_shadow(theme: &Theme, status: button::Status) -> Shadow {
    let palette = theme.extended_palette();
    if matches!(status, button::Status::Pressed) {
        Default::default()
    } else {
        Shadow {
            color: palette.secondary.strong.color,
            offset: [2.0, 2.0].into(),
            blur_radius: 1.0,
        }
    }
}
