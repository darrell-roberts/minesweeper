//! Minesweeper application state view and updates.
use iced::{
    animation::Easing,
    border, color, time,
    widget::{button, column, container, opaque, row, text, Column, Row},
    window, Animation, Color, Element, Length, Subscription, Task,
};
use minesweeper::{
    history::{load_wins, save_win, WinHistory},
    model::{Board, CellState, GameState, Pos},
};
use modal::modal;
use std::{
    num::NonZeroU8,
    time::{Duration, Instant},
};
use views::{cell_view, CellView, Header, ScoreBoard};

mod modal;
mod views;

pub struct AppState {
    pub board: Board,
    elapsed_seconds: u64,
    outcome: Option<String>,
    scoreboard: Option<WinHistory>,
    cells: Vec<CellView>,
    now: Instant,
    modal_animation: Animation<bool>,
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
    Animate,
}

impl AppState {
    pub fn new() -> Self {
        let board = Board::new(
            NonZeroU8::try_from(20).unwrap(),
            NonZeroU8::try_from(20).unwrap(),
        );
        Self {
            cells: board
                .positions()
                .map(|(pos, cell)| cell_view(*cell, *pos, *board.state()))
                .collect(),
            board,
            elapsed_seconds: 0,
            outcome: None,
            scoreboard: None,
            now: Instant::now(),
            modal_animation: mk_modal_animation(),
        }
    }

    pub fn subscription(&self) -> Subscription<AppMsg> {
        let is_animating = self
            .cells
            .iter()
            .any(|cell| cell.animated.is_animating(self.now))
            || self.modal_animation.is_animating(self.now);

        Subscription::batch([
            if matches!(self.board.state(), GameState::Active) {
                time::every(Duration::from_secs(1)).map(|_| AppMsg::Tick)
            } else {
                Subscription::none()
            },
            if is_animating {
                window::frames().map(|_| AppMsg::Animate)
            } else {
                Subscription::none()
            },
        ])
    }

    pub fn update(&mut self, message: AppMsg, instant: Instant) -> Task<AppMsg> {
        self.now = instant;
        self.cells.iter_mut().for_each(|cell_view| {
            cell_view.instant = instant;
            cell_view.game_state = *self.board.state();
        });

        match message {
            AppMsg::Open(pos)
                if matches!(self.board.state(), GameState::Active | GameState::New) =>
            {
                self.board.open_cell(pos);

                // Update cell state.
                for (cell_view, (_pos, cell)) in self.cells.iter_mut().zip(self.board.positions()) {
                    // Enable open animation for all opened cells.
                    if let (CellState::Closed { .. }, CellState::Open) =
                        (cell_view.cell.state, cell.state)
                    {
                        cell_view.open();
                    }

                    if let (CellState::Closed { .. }, CellState::ExposedMine) =
                        (cell_view.cell.state, cell.state)
                    {
                        cell_view.boom();
                    }
                    cell_view.cell = *cell;
                }

                match self.board.state() {
                    GameState::Loss => {
                        self.outcome = Some("You lose!".into());
                        self.modal_animation.go_mut(true, self.now);
                    }
                    GameState::Win => {
                        self.outcome = Some("You won!".into());
                        self.modal_animation.go_mut(true, self.now);
                        if let Err(err) = save_win(self.elapsed_seconds) {
                            eprintln!("Failed to save win: {err}");
                        }
                    }
                    _ => (),
                };
            }
            AppMsg::Flag(pos) if matches!(self.board.state(), GameState::Active) => {
                self.board.flag_cell(pos);

                // Update cell state.
                for (cell_view, (_pos, cell)) in self.cells.iter_mut().zip(self.board.positions()) {
                    // Enabled flagged animation for flagged cell.
                    if let (
                        CellState::Closed { flagged: false, .. },
                        CellState::Closed { flagged: true, .. },
                    ) = (cell_view.cell.state, cell.state)
                    {
                        cell_view.flag();
                    }
                    cell_view.cell = *cell;
                }
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
                self.cells = self
                    .board
                    .positions()
                    .map(|(pos, cell)| cell_view(*cell, *pos, *self.board.state()))
                    .collect();
                self.outcome = None;
                self.modal_animation = mk_modal_animation();
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
            _ => (),
        }
        Task::none()
    }

    pub fn view(&self) -> iced::Element<'_, AppMsg> {
        let mut y = 1;
        let mut rows = Vec::new();
        let mut row: Vec<Element<'_, AppMsg>> = Vec::new();

        for cell_view in &self.cells {
            if cell_view.pos.y.get() != y {
                rows.push(Element::from(Row::with_children(row).spacing(2)));
                row = Vec::new();
                y = cell_view.pos.y.get();
            }

            row.push(cell_view.view());
        }

        rows.push(Element::from(Row::with_children(row).spacing(2)));

        let button_row = row![
            container(
                button("Restart")
                    .style(|theme, status| button::Style {
                        border: border::rounded(10),
                        ..button::primary(theme, status)
                    })
                    .on_press(AppMsg::Restart)
            )
            .padding(10),
            container(
                button("Scoreboard")
                    .style(|theme, status| button::Style {
                        border: border::rounded(10),
                        ..button::primary(theme, status)
                    })
                    .on_press(AppMsg::ViewScoreBoard),
            )
            .padding(10),
        ];

        let button_container = container(button_row)
            .width(Length::Fill)
            .center_x(Length::Fill)
            .padding(20);

        let board = container(Column::with_children(rows).spacing(2))
            // .width(Length::Fill)
            // .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(|theme| container::dark(theme).background(color!(0xf2f2f2)));

        let content = column![
            Header::new(&self.board, self.elapsed_seconds).view(),
            if matches!(self.board.state(), GameState::Loss | GameState::Win) {
                opaque(board)
            } else {
                board.into()
            },
            button_container
        ];

        if let Some(outcome) = self.outcome.as_ref() {
            modal(
                content,
                container(text(outcome).size(30))
                    .center_x(Length::Fill)
                    .padding(20)
                    .width(200)
                    .style(|theme| modal_content_style(theme, &self.modal_animation, self.now)),
                AppMsg::DismissModal,
            )
        } else if let Some(wins) = self.scoreboard.as_ref() {
            modal(
                content,
                container(ScoreBoard::new(&wins.wins).view())
                    .padding(10)
                    .style(|theme| modal_content_style(theme, &self.modal_animation, self.now)),
                AppMsg::DismissScoreBoard,
            )
        } else {
            content.into()
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

fn modal_content_style(
    theme: &iced::Theme,
    animation: &Animation<bool>,
    now: Instant,
) -> container::Style {
    container::dark(theme)
        .border(iced::border::rounded(15))
        .background(Color {
            a: if animation.is_animating(now) {
                animation.interpolate(0.2, 1.0, now)
            } else {
                0.5
            },
            ..Color::BLACK
        })
        .color(Color {
            a: if animation.is_animating(now) {
                animation.interpolate(0.2, 1.0, now)
            } else {
                1.0
            },
            ..Color::WHITE
        })
}

fn mk_modal_animation() -> Animation<bool> {
    Animation::new(false)
        .easing(Easing::EaseInBack)
        .very_slow()
        .repeat(2)
}
