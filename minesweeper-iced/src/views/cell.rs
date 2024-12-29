use crate::AppMsg;
use iced::{
    widget::{button, container, mouse_area, text, Button},
    Color, Element, Length,
};
use minesweeper::model::{Cell, CellState, GameState, Pos};

pub struct CellView {
    cell: Cell,
    pos: Pos,
    game_state: GameState,
}

impl CellView {
    pub fn new(cell: Cell, pos: Pos, game_state: GameState) -> Self {
        Self {
            cell,
            pos,
            game_state,
        }
    }
}

pub fn cell_view(cell: Cell, pos: Pos, game_state: GameState) -> CellView {
    CellView::new(cell, pos, game_state)
}

impl CellView {
    pub fn view<'a>(&self) -> Element<'a, AppMsg> {
        let adjacent_mines = self.cell.adjacent_mines;

        let content: Element<'a, AppMsg> = match self.cell.state {
            CellState::Open => {
                if self.cell.adjacent_mines > 0 {
                    container(
                        text(format!("{adjacent_mines}"))
                            .center()
                            .style(move |_| select_color(adjacent_mines))
                            .center(),
                    )
                    .center(Length::Fill)
                    .into()
                } else {
                    text("").into()
                }
            }
            CellState::Closed { flagged, .. } => {
                let game_active = matches!(self.game_state, GameState::Active | GameState::New);
                if flagged {
                    mouse_area(cell_button(
                        text("🚩").shaping(text::Shaping::Advanced).center(),
                    ))
                    .on_right_press(if game_active {
                        AppMsg::Flag(self.pos)
                    } else {
                        AppMsg::None
                    })
                    .into()
                } else {
                    mouse_area(
                        cell_button("")
                            .on_press_maybe(game_active.then_some(AppMsg::Open(self.pos))),
                    )
                    .on_right_press(if game_active {
                        AppMsg::Flag(self.pos)
                    } else {
                        AppMsg::None
                    })
                    .into()
                }
            }
            CellState::ExposedMine => {
                container(text("💣").shaping(text::Shaping::Advanced).center())
                    .center(Length::Fill)
                    .into()
            }
        };

        container(content).width(45).height(45).into()
    }
}

fn select_color(adjacent_mines: u8) -> text::Style {
    text::Style {
        color: Some(match adjacent_mines {
            1 => Color::WHITE,
            2 => Color::from_rgb8(0, 229, 0),
            3 => Color::from_rgb8(230, 118, 0),
            _ => Color::from_rgb8(254, 0, 0),
        }),
    }
}

fn cell_button<'a, Message>(content: impl Into<Element<'a, Message>>) -> Button<'a, Message>
where
    Message: Clone + 'a,
{
    button(content).width(45).height(45)
}
