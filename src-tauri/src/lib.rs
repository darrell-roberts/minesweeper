use game::Game;
use serde::Serialize;
use std::sync::{Arc, RwLock};

pub mod commands;
pub mod game;
pub mod history;

#[derive(Serialize, Clone)]
pub struct TimeEvent {
  pub duration: String,
}

pub type AppGame = Arc<RwLock<Game>>;

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
