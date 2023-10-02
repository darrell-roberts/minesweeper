use iced::{window, Application, Settings};
use minesweeper_iced::AppState;

fn main() -> iced::Result {
  let settings = Settings {
    window: window::Settings {
      size: (600, 700),
      resizable: false,
      ..Default::default()
    },
    ..Default::default()
  };
  AppState::run(settings)
}
