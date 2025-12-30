//! Minesweeper application state view and updates.
use iced::{
    Animation, Color, Element, Length, Subscription, Task,
    animation::Easing,
    border, color, time,
    widget::{Column, Row, button, column, container, row, text},
    window,
};
use minesweeper::{
    history::{WinHistory, load_wins, save_win},
    model::{Board, CellState, GameState, Pos},
};
use modal::modal;
use std::{
    num::NonZeroU8,
    time::{Duration, Instant},
};
use views::{CellView, Header, ScoreBoard, cell_view};

mod modal;
mod views;

/// Application state.
pub struct AppState {
    /// Game board.
    pub board: Board,
    /// Active play timer.
    elapsed_seconds: u64,
    /// Win outcome.
    outcome: Option<String>,
    /// Scoreboard when viewing historic wins.
    scoreboard: Option<WinHistory>,
    /// Game cells.
    cells: Vec<CellView>,
    /// Current instant.
    now: Instant,
    /// Animation for modal.
    modal_animation: Animation<bool>,
}

/// Application messages.
#[derive(Debug, Copy, Clone)]
pub enum AppMsg {
    /// Open a cell via its position.
    Open(Pos),
    /// Flag a cell via its position.
    Flag(Pos),
    /// Timer tick.
    Tick,
    /// Restart the game.
    Restart,
    /// Close the open modal.
    DismissModal,
    /// View the scoreboard.
    ViewScoreBoard,
    /// Dismiss the scoreboard.
    DismissScoreBoard,
    /// No-op.
    None,
    /// Render for animation. No-op.
    Animate,
}

impl AppState {
    /// Create a new application state.
    fn new() -> Self {
        let board = Board::new(
            NonZeroU8::try_from(20).unwrap(),
            NonZeroU8::try_from(20).unwrap(),
        );
        let now = Instant::now();
        Self {
            cells: board
                .positions()
                .map(|(pos, cell)| cell_view(*cell, *pos, *board.state(), now))
                .collect(),
            board,
            elapsed_seconds: 0,
            outcome: None,
            scoreboard: None,
            now,
            modal_animation: mk_modal_animation(),
        }
    }

    /// Application subscriptions for timer and animations.
    pub fn subscription(&self) -> Subscription<AppMsg> {
        // Check if any of our animations are active.
        let is_animating = self.cells.iter().any(|cell| cell.is_animating(self.now))
            || self.modal_animation.is_animating(self.now);

        fn maybe_subscription<T>(b: bool, f: impl FnOnce() -> Subscription<T>) -> Subscription<T> {
            if b { f() } else { Subscription::none() }
        }

        Subscription::batch([
            maybe_subscription(matches!(self.board.state(), GameState::Active), || {
                time::every(Duration::from_secs(1)).map(|_| AppMsg::Tick)
            }),
            maybe_subscription(is_animating, || window::frames().map(|_| AppMsg::Animate)),
        ])
    }

    /// Update game application view state.
    pub fn update(&mut self, message: AppMsg, instant: Instant) -> Task<AppMsg> {
        self.now = instant;
        self.cells.iter_mut().for_each(|cell_view| {
            cell_view.now = instant;
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

                if matches!(self.board.state(), GameState::Win) {
                    self.outcome = Some("You won!".into());
                    self.modal_animation.go_mut(true, self.now);
                    if let Err(err) = save_win(self.elapsed_seconds) {
                        eprintln!("Failed to save win: {err}");
                    }
                }
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
                    .map(|(pos, cell)| cell_view(*cell, *pos, *self.board.state(), self.now))
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

    /// Render the game view.
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
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(|theme| container::primary(theme).background(color!(0xf2f2f2)));

        let content = column![
            Header::new(&self.board, self.elapsed_seconds).view(),
            board,
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
    container::primary(theme)
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
