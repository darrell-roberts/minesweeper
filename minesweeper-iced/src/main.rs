use iced::{time, window, Subscription, Theme};
use minesweeper::model::GameState;
use minesweeper_iced::{AppMsg, AppState};
use std::time::Duration;

fn main() -> iced::Result {
    launch()
}

fn launch() -> iced::Result {
    iced::application("Minesweeper", AppState::update, AppState::view)
        .subscription(|state| {
            if matches!(state.board.state(), GameState::Active) {
                time::every(Duration::from_secs(1)).map(|_| AppMsg::Tick)
            } else {
                Subscription::none()
            }
        })
        .theme(|_state| Theme::Dark)
        .window(window::Settings {
            size: (900., 900.).into(),
            #[cfg(target_os = "linux")]
            platform_specific: window::settings::PlatformSpecific {
                application_id: "minesweeper".into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .run_with(AppState::new)
}
