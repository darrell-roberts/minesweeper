use iced::{window, Application, Settings};
use minesweeper_iced::AppState;

fn main() -> iced::Result {
  let settings = Settings {
    window: window::Settings {
      size: (500, 650),
      ..Default::default()
    },
    ..Default::default()
  };
  AppState::run(settings)
}
