use iced::{time, Subscription, Theme};
use minesweeper::model::GameState;
use minesweeper_iced::{AppMsg, AppState};
use std::time::Duration;

fn main() -> iced::Result {
    launch()
}

fn launch() -> iced::Result {
    // let settings = Settings {
    //     window: window::Settings {
    //         size: (600, 700),
    //         resizable: false,
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // };
    iced::application("Minesweeper", AppState::update, AppState::view)
        .subscription(|state| {
            if matches!(state.board.state(), GameState::Active) {
                time::every(Duration::from_secs(1)).map(|_| AppMsg::Tick)
            } else {
                Subscription::none()
            }
        })
        .theme(|_state| Theme::Dark)
        .run_with(AppState::new)
}
