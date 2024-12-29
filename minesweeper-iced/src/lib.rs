use iced::{
    widget::{button, column, container, row, text, Column, Row},
    Color, Element, Length, Task,
};
use minesweeper::{
    history::{load_wins, save_win, WinHistory},
    model::{Board, GameState, Pos},
};
use modal::modal;
use std::num::NonZeroU8;
use views::{cell_view, Header, ScoreBoard};

mod modal;
mod views;

pub struct AppState {
    pub board: Board,
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
    None,
}

impl AppState {
    pub fn new() -> (AppState, Task<AppMsg>) {
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
            Task::none(),
        )
    }

    pub fn update(&mut self, message: AppMsg) -> Task<AppMsg> {
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
            AppMsg::None => (),
        }
        Task::none()
    }

    pub fn view(&self) -> iced::Element<AppMsg> {
        let mut y = 1;
        let mut rows = Vec::new();
        let mut row: Vec<Element<AppMsg>> = Vec::new();

        for (pos, cell) in self.board.positions() {
            if pos.y.get() != y {
                rows.push(Element::from(Row::with_children(row).spacing(2)));
                row = Vec::new();
                y = pos.y.get();
            }

            row.push(cell_view(*cell, *pos, *self.board.state()).view());
        }

        rows.push(Element::from(Row::with_children(row).spacing(2)));

        let button_row = row![
            container(button("Restart").on_press(AppMsg::Restart)).padding(10),
            container(button("Scoreboard").on_press(AppMsg::ViewScoreBoard),).padding(10),
        ];

        let button_container = container(button_row)
            .width(Length::Fill)
            .center_x(Length::Fill)
            .padding(20);

        let content = column![
            Header::new(&self.board, self.elapsed_seconds).view(),
            container(Column::with_children(rows).spacing(2))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill),
            button_container
        ];

        if let Some(outcome) = self.outcome.as_ref() {
            modal(
                content,
                container(text(outcome))
                    .center_x(Length::Fill)
                    .padding(20)
                    .width(200)
                    .style(modal_content_style),
                AppMsg::DismissModal,
            )
        } else if let Some(wins) = self.scoreboard.as_ref() {
            modal(
                content,
                container(ScoreBoard::new(&wins.wins).view())
                    .padding(10)
                    .style(modal_content_style),
                AppMsg::DismissScoreBoard,
            )
        } else {
            content.into()
        }
    }
}

fn modal_content_style(_theme: &iced::Theme) -> container::Style {
    container::Style::default()
        .background(Color::from_rgba8(0, 153, 204, 0.7))
        .border(iced::border::rounded(15))
}
