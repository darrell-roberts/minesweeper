use iced::{time, Subscription, Theme};
use minesweeper::model::GameState;
use minesweeper_iced::{AppMsg, AppState};
use std::time::Duration;

fn main() -> iced::Result {
    launch()
}

fn launch() -> iced::Result {
    iced::application("Minesweeper", AppState::update, AppState::view)
        .window_size((1024., 1124.))
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
