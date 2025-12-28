use iced::{window, Theme};
use minesweeper_iced::AppState;

fn main() -> iced::Result {
    launch()
}

fn launch() -> iced::Result {
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
    .theme(|_app: &AppState| Theme::Dark)
    .run()
}
