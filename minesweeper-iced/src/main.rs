//! A simple minesweeper game.
use iced::window;
use minesweeper_iced::AppState;

fn main() -> iced::Result {
    iced::application::timed(
        AppState::default,
        AppState::update,
        AppState::subscription,
        AppState::view,
    )
    .title("Minesweeper")
    .window(window::Settings {
        size: (900., 900.).into(),
        #[cfg(target_os = "linux")]
        platform_specific: window::settings::PlatformSpecific {
            application_id: "io.github.darrellroberts.minesweeper".into(),
            ..Default::default()
        },
        ..Default::default()
    })
    .theme(|app: &AppState| app.theme.clone())
    .run()
}
