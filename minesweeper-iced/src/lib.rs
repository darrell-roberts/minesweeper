use components::{cell_component, Header};
use iced::{
  executor, time,
  widget::{button, column, container, Column, Row},
  Application, Command, Element, Length, Subscription, Theme,
};
use minesweeper::model::{Board, GameState, Pos};
use std::{num::NonZeroU8, time::Duration};

mod components;

pub struct AppState {
  board: Board,
  elapsed_seconds: u64,
}

#[derive(Debug, Copy, Clone)]
pub enum AppMsg {
  Open(Pos),
  Flag(Pos),
  Tick,
  Restart,
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
      }
      AppMsg::Flag(pos) => {
        self.board.flag_cell(pos);
      }
      AppMsg::Tick => {
        self.elapsed_seconds += 1;
      }
      AppMsg::Restart => {
        self.elapsed_seconds = 0;
        self.board = Board::new(
          NonZeroU8::try_from(20).unwrap(),
          NonZeroU8::try_from(20).unwrap(),
        )
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

    let restart_button = container(button("Restart").on_press(AppMsg::Restart))
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
      restart_button
    ];

    content.into()
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
