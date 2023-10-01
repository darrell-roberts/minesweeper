use crate::Command;
use iced::{
  widget::{button, component, text, Component},
  Element, Renderer,
};
use minesweeper::model::{Cell, CellState, Pos};

pub struct CellComponent<Message> {
  cell: Cell,
  pos: Pos,
  on_change: Box<dyn Fn(Command) -> Message>,
}

impl<Message> CellComponent<Message> {
  pub fn new(
    cell: Cell,
    pos: Pos,
    on_change: impl Fn(Command) -> Message + 'static,
  ) -> Self {
    Self {
      cell,
      pos,
      on_change: Box::new(on_change),
    }
  }
}

pub fn cell_component<Message>(
  cell: Cell,
  pos: Pos,
  on_change: impl Fn(Command) -> Message + 'static,
) -> CellComponent<Message> {
  CellComponent::new(cell, pos, on_change)
}

#[derive(Debug, Copy, Clone)]
pub enum CellEvent {
  Open,
  Flag,
}

impl<Message> Component<Message, Renderer> for CellComponent<Message> {
  type State = Cell;

  type Event = CellEvent;

  fn update(
    &mut self,
    _state: &mut Self::State,
    event: Self::Event,
  ) -> Option<Message> {
    match event {
      CellEvent::Open => Some((self.on_change)(Command::Open(self.pos))),
      CellEvent::Flag => Some((self.on_change)(Command::Flag(self.pos))),
    }
  }

  fn view(&self, _state: &Self::State) -> Element<'_, Self::Event, Renderer> {
    match self.cell.state {
      CellState::Open => {
        if self.cell.adjacent_mines > 0 {
          button(text(format!("{}", self.cell.adjacent_mines)))
        } else {
          button(" ")
        }
      }
      CellState::Closed { flagged, .. } => {
        if flagged {
          button("🚩").on_press(CellEvent::Flag)
        } else {
          button("").on_press(CellEvent::Open)
        }
      }
      CellState::ExposedMine => button("💣"),
    }
    .padding(10)
    .width(35)
    .into()
  }
}

impl<'a, Message> From<CellComponent<Message>>
  for Element<'a, Message, Renderer>
where
  Message: 'a,
{
  fn from(value: CellComponent<Message>) -> Self {
    component(value)
  }
}
