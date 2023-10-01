use components::{cell_component, Header};
use iced::{
  alignment::{Horizontal, Vertical},
  widget::{column, container, Column, Row},
  Element, Length, Sandbox,
};
use minesweeper::model::{Board, Pos};
use std::num::NonZeroU8;

mod components;

pub struct AppState {
  board: Board,
}

#[derive(Debug, Copy, Clone)]
pub enum Command {
  Open(Pos),
  Flag(Pos),
}

impl Sandbox for AppState {
  type Message = Command;

  fn new() -> Self {
    Self {
      board: Board::new(
        NonZeroU8::try_from(10).unwrap(),
        NonZeroU8::try_from(10).unwrap(),
      ),
    }
  }

  fn title(&self) -> String {
    String::from("Minesweeper Iced")
  }

  fn update(&mut self, message: Self::Message) {
    match message {
      Command::Open(pos) => {
        self.board.open_cell(pos);
      }
      Command::Flag(pos) => {
        self.board.flag_cell(pos);
      }
    }
  }

  fn view(&self) -> iced::Element<Self::Message> {
    let mut y = 1;
    let mut rows = Vec::new();
    let mut row: Vec<Element<Command>> = Vec::new();

    for (pos, cell) in self.board.positions() {
      if pos.y.get() != y {
        rows.push(Element::from(Row::with_children(row).spacing(10)));
        row = Vec::new();
        y = pos.y.get();
      }

      row.push(Element::from(cell_component(*cell, *pos, |command| {
        command
      })));
    }

    rows.push(Element::from(Row::with_children(row).spacing(10)));

    let header = container(Header::new(&self.board))
      .align_x(Horizontal::Center)
      .align_y(Vertical::Center);

    let content = column![header, Column::with_children(rows).spacing(10)];

    container(content)
      .width(Length::Fill)
      .height(Length::Fill)
      .center_x()
      .center_y()
      .into()
  }
}
