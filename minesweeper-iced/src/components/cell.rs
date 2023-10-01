use crate::AppMsg;
use iced::{
  alignment::Horizontal,
  font::Weight,
  theme,
  widget::{button, component, text, Component},
  Color, Element, Font, Renderer,
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
    let bold_font = || Font {
      weight: Weight::Bold,
      ..Default::default()
    };
    match self.cell.state {
      CellState::Open => {
        if self.cell.adjacent_mines > 0 {
          button(
            text(format!("{}", self.cell.adjacent_mines))
              .horizontal_alignment(Horizontal::Center)
              .style(select_color(self.cell.adjacent_mines))
              .font(bold_font()),
          )
        } else {
          button(" ")
        }
      }
      CellState::Closed { flagged, .. } => {
        let game_active =
          matches!(self.game_state, GameState::Active | GameState::New);
        if flagged {
          button(text("F").horizontal_alignment(Horizontal::Center)).on_press(
            if game_active {
              CellEvent::Flag
            } else {
              CellEvent::None
            },
          )
        } else {
          button("").on_press(if game_active {
            CellEvent::Open
          } else {
            CellEvent::None
          })
        }
      }
      CellState::ExposedMine => button(
        text("X")
          .horizontal_alignment(Horizontal::Center)
          .font(bold_font())
          .style(theme::Text::Color(Color {
            r: 217. / 255.,
            g: 0.,
            b: 0.,
            a: 1.,
          })),
      ),
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

fn select_color(adjacent_mines: u8) -> theme::Text {
  match adjacent_mines {
    1 => theme::Text::Color(Color::WHITE),
    2 => theme::Text::Color(Color {
      r: 0.,
      g: 229. / 256.,
      b: 0.,
      a: 1.,
    }),
    3 => theme::Text::Color(Color {
      r: 230. / 256.,
      g: 118. / 256.,
      b: 0.,
      a: 1.,
    }),
    _ => theme::Text::Color(Color {
      r: 254. / 255.,
      g: 0.,
      b: 0.,
      a: 1.,
    }),
  }
}
