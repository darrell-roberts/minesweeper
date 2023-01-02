use components::{cell_component, Header, ScoreBoard};
use iced::{
    executor, time,
    widget::{button, column, container, row, text, Column, Row},
    Application, Command, Element, Length, Subscription, Theme,
};
use minesweeper::{
    history::{load_wins, save_win, WinHistory},
    model::{Board, GameState, Pos},
};
use std::{num::NonZeroU8, time::Duration};
use theme::ModalStyle;
use widgets::Modal;

mod components;
mod theme;
mod widgets;

pub struct AppState {
    board: Board,
    elapsed_seconds: u64,
    outcome: Option<String>,
    scoreboard: Option<WinHistory>,
}

#[derive(Debug, Copy, Clone)]
pub enum AppMsg {
    Open(Pos),
    Flag(Pos),
    Tick,
    Restart,
    DismissModal,
    ViewScoreBoard,
    DismissScoreBoard,
}

impl Application for AppState {
    type Executor = executor::Default;
    type Flags = ();
    type Message = AppMsg;
    type Theme = Theme;

    fn new(_flags: ()) -> (AppState, Command<Self::Message>) {
        (
            Self {
                board: Board::new(
                    NonZeroU8::try_from(20).unwrap(),
                    NonZeroU8::try_from(20).unwrap(),
                ),
                elapsed_seconds: 0,
                outcome: None,
                scoreboard: None,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Minesweeper")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            AppMsg::Open(pos) => {
                self.board.open_cell(pos);
                match self.board.state() {
                    GameState::Loss => self.outcome = Some("You lose!".into()),
                    GameState::Win => {
                        self.outcome = Some("You win!".into());
                        if let Err(err) = save_win(self.elapsed_seconds) {
                            eprintln!("Failed to save win: {err}");
                        }
                    }
                    _ => (),
                };
            }
            AppMsg::Flag(pos) => {
                self.board.flag_cell(pos);
            }
            AppMsg::Tick => {
                // Pause timer when viewing scoreboard.
                if self.scoreboard.is_none() {
                    self.elapsed_seconds += 1;
                }
            }
            AppMsg::Restart => {
                self.elapsed_seconds = 0;
                self.board = Board::new(
                    NonZeroU8::try_from(20).unwrap(),
                    NonZeroU8::try_from(20).unwrap(),
                );
                self.outcome = None;
            }
            AppMsg::DismissModal => {
                self.outcome = None;
            }
            AppMsg::ViewScoreBoard => {
                self.scoreboard = load_wins();
            }
            AppMsg::DismissScoreBoard => {
                self.scoreboard = None;
            }
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<Self::Message> {
        let mut y = 1;
        let mut rows = Vec::new();
        let mut row: Vec<Element<AppMsg>> = Vec::new();

        for (pos, cell) in self.board.positions() {
            if pos.y.get() != y {
                rows.push(Element::from(Row::with_children(row).spacing(2)));
                row = Vec::new();
                y = pos.y.get();
            }

            row.push(Element::from(cell_component(
                *cell,
                *pos,
                *self.board.state(),
                |command| command,
            )));
        }

        rows.push(Element::from(Row::with_children(row).spacing(2)));

        let button_row = row![
            container(button("Restart").on_press(AppMsg::Restart)).padding(10),
            container(button("Scoreboard").on_press(AppMsg::ViewScoreBoard),)
                .padding(10),
        ];

        let button_container = container(button_row)
            .width(Length::Fill)
            .center_x()
            .padding(20);

        let content = column![
            Header::new(&self.board, self.elapsed_seconds),
            container(Column::with_children(rows).spacing(2))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y(),
            button_container
        ];

        if let Some(outcome) = self.outcome.as_ref() {
            let modal = container(text(outcome))
                .center_x()
                .padding(20)
                .width(200)
                .style(iced::theme::Container::Custom(Box::new(ModalStyle)));
            Modal::new(content, modal)
                .on_blur(AppMsg::DismissModal)
                .into()
        } else if let Some(wins) = self.scoreboard.as_ref() {
            let modal = container(ScoreBoard::new(&wins.wins))
                .padding(10)
                .style(iced::theme::Container::Custom(Box::new(ModalStyle)));
            Modal::new(content, modal)
                .on_blur(AppMsg::DismissScoreBoard)
                .into()
        } else {
            content.into()
        }
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        if matches!(self.board.state(), GameState::Active) {
            time::every(Duration::from_secs(1)).map(|_| AppMsg::Tick)
        } else {
            Subscription::none()
        }
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}
