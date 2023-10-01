use iced::{
  widget::{component, row, text, Component},
  Element, Renderer,
};
use minesweeper::model::Board;

pub struct Header {
  time_elapsed: u64,
  opened: usize,
  flagged: usize,
  mined: usize,
}

impl Header {
  pub fn new(board: &Board) -> Self {
    Self {
      time_elapsed: 0,
      opened: board.opened(),
      flagged: board.flagged(),
      mined: board.mined(),
    }
  }
}

#[derive(Debug, Copy, Clone)]
pub enum HeaderEvent {}

impl<Message> Component<Message, Renderer> for Header {
  type State = ();

  type Event = HeaderEvent;

  fn update(
    &mut self,
    _state: &mut Self::State,
    _event: Self::Event,
  ) -> Option<Message> {
    todo!()
  }

  fn view(&self, _state: &Self::State) -> Element<'_, Self::Event, Renderer> {
    row![
      text(format!("Opened {}", self.opened)),
      text(format!("Flagged {}", self.flagged)),
      text(format!("Mined {}", self.mined))
    ]
    .spacing(20)
    .padding(10)
    .into()
  }
}

impl<'a, Message> From<Header> for Element<'a, Message, Renderer>
where
  Message: 'a,
{
  fn from(value: Header) -> Self {
    component(value)
  }
}
