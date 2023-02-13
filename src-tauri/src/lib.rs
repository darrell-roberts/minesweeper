use game::Game;
use serde::Serialize;
use std::sync::{Arc, RwLock};

pub mod commands;
pub mod game;

#[derive(Serialize, Clone)]
pub struct TimeEvent {
  pub duration: String,
}

pub type AppGame = Arc<RwLock<Game>>;
