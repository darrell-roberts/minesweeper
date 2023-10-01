use iced::{window, Sandbox, Settings};
use minesweeper_iced::AppState;

fn main() -> iced::Result {
  let settings = Settings {
    window: window::Settings {
      size: (500, 600),
      ..Default::default()
    },
    // default_font: Font {
    //   family: Family::Name("Fira Code Retina"),
    //   ..Default::default()
    // },
    ..Default::default()
  };
  AppState::run(settings)
}
