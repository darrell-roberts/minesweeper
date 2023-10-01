use crate::AppMsg;
use iced::{
  widget::{button, component, text, Component},
  Element, Renderer,
};
use minesweeper::model::{Cell, CellState, GameState, Pos};

pub struct CellComponent<Message> {
  cell: Cell,
  pos: Pos,
  game_state: GameState,
  on_change: Box<dyn Fn(AppMsg) -> Message>,
}

impl<Message> CellComponent<Message> {
  pub fn new(
    cell: Cell,
    pos: Pos,
    game_state: GameState,
    on_change: impl Fn(AppMsg) -> Message + 'static,
  ) -> Self {
    Self {
      cell,
      pos,
      game_state,
      on_change: Box::new(on_change),
    }
  }
}

pub fn cell_component<Message>(
  cell: Cell,
  pos: Pos,
  game_state: GameState,
  on_change: impl Fn(AppMsg) -> Message + 'static,
) -> CellComponent<Message> {
  CellComponent::new(cell, pos, game_state, on_change)
}

#[derive(Debug, Copy, Clone)]
pub enum CellEvent {
  Open,
  Flag,
  None,
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
      CellEvent::Open => Some((self.on_change)(AppMsg::Open(self.pos))),
      CellEvent::Flag => Some((self.on_change)(AppMsg::Flag(self.pos))),
      CellEvent::None => None,
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
        let game_active =
          matches!(self.game_state, GameState::Active | GameState::New);
        if flagged {
          button("🚩").on_press(if game_active {
            CellEvent::Flag
          } else {
            CellEvent::None
          })
        } else {
          button("").on_press(if game_active {
            CellEvent::Open
          } else {
            CellEvent::None
          })
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
