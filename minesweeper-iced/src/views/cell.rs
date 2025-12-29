//! View for a single cell.
use std::time::Instant;

use crate::AppMsg;
use iced::{
    animation::Easing,
    widget::{button, container, mouse_area, text, Button},
    Animation, Color, Element, Length,
};
use minesweeper::model::{Cell, CellState, GameState, Pos};

pub struct CellView {
    pub cell: Cell,
    pub pos: Pos,
    pub game_state: GameState,
    pub animated: Animation<bool>,
    pub instant: Instant,
}

impl CellView {
    fn new(cell: Cell, pos: Pos, game_state: GameState) -> Self {
        Self {
            cell,
            pos,
            game_state,
            animated: mk_cell_animation(),
            instant: Instant::now(),
        }
    }
}

pub fn cell_view(cell: Cell, pos: Pos, game_state: GameState) -> CellView {
    CellView::new(cell, pos, game_state)
}

fn mk_cell_animation() -> Animation<bool> {
    Animation::new(false).easing(Easing::EaseIn).quick()
}

impl CellView {
    pub fn open(&mut self, now: Instant) {
        self.instant = now;
        self.animated.go_mut(true, self.instant);
    }

    pub fn flag(&mut self, now: Instant) {
        self.instant = now;
        if self.animated.value() {
            self.animated = mk_cell_animation();
        }
        self.animated.go_mut(true, self.instant);
    }

    pub fn view(&self) -> Element<'_, AppMsg> {
        let adjacent_mines = self.cell.adjacent_mines;

        let content: Element<'_, AppMsg> = match self.cell.state {
            CellState::Open => {
                if self.cell.adjacent_mines > 0 {
                    container(
                        text(format!("{adjacent_mines}"))
                            .center()
                            .style(move |_| {
                                if self.animated.is_animating(self.instant) {
                                    select_color(
                                        adjacent_mines,
                                        self.animated.interpolate(0.0, 1.0, self.instant),
                                    )
                                } else {
                                    select_color(adjacent_mines, 1.0)
                                }
                            })
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
                    mouse_area(
                        cell_button(text("🚩").shaping(text::Shaping::Advanced).center()).style(
                            |theme, status| {
                                let mut style = button::primary(theme, status);
                                let background =
                                    style.background.map(|background| match background {
                                        iced::Background::Color(mut color) => {
                                            color.a = if self.animated.is_animating(self.instant) {
                                                self.animated.interpolate(0.0, 1.0, self.instant)
                                            } else {
                                                1.0
                                            };

                                            iced::Background::Color(color)
                                        }
                                        iced::Background::Gradient(gradient) => {
                                            iced::Background::Gradient(gradient)
                                        }
                                    });
                                style.background = background;
                                style.text_color.a = if self.animated.is_animating(self.instant) {
                                    self.animated.interpolate(0.0, 1.0, self.instant)
                                } else {
                                    1.0
                                };
                                style
                            },
                        ),
                    )
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

        container(content).width(35).height(35).into()
    }
}

fn select_color(adjacent_mines: u8, opacity: f32) -> text::Style {
    text::Style {
        color: Some(Color {
            a: opacity,
            ..match adjacent_mines {
                1 => Color::WHITE,
                2 => Color::from_rgb8(0, 229, 0),
                3 => Color::from_rgb8(230, 118, 0),
                _ => Color::from_rgb8(254, 0, 0),
            }
        }),
    }
}

fn cell_button<'a, Message>(content: impl Into<Element<'a, Message>>) -> Button<'a, Message>
where
    Message: Clone + 'a,
{
    button(content).width(35).height(35)
}
