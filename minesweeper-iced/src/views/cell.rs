//! View for a single cell.
use std::time::Instant;

use crate::AppMsg;
use iced::{
    Animation, Color, Element, Length,
    animation::Easing,
    color,
    widget::{Button, button, container, mouse_area, text},
};
use minesweeper::model::{Cell, CellState, GameState, Pos};

pub struct CellView {
    pub cell: Cell,
    pub pos: Pos,
    pub game_state: GameState,
    cell_animation: Animation<bool>,
    exposed_animation: Animation<bool>,
    pub now: Instant,
}

impl CellView {
    fn new(cell: Cell, pos: Pos, game_state: GameState, now: Instant) -> Self {
        Self {
            cell,
            pos,
            game_state,
            cell_animation: mk_cell_animation(),
            exposed_animation: Animation::new(false)
                .repeat(3)
                .easing(Easing::EaseIn)
                .slow(),
            now,
        }
    }

    pub fn is_animating(&self, now: Instant) -> bool {
        self.cell_animation.is_animating(now) || self.exposed_animation.is_animating(now)
    }
}

pub fn cell_view(cell: Cell, pos: Pos, game_state: GameState, now: Instant) -> CellView {
    CellView::new(cell, pos, game_state, now)
}

fn mk_cell_animation() -> Animation<bool> {
    Animation::new(false).easing(Easing::EaseIn).quick()
}

impl CellView {
    pub fn open(&mut self) {
        self.cell_animation.go_mut(true, self.now);
    }

    pub fn flag(&mut self) {
        if self.cell_animation.value() {
            self.cell_animation = mk_cell_animation();
        }
        self.cell_animation.go_mut(true, self.now);
    }

    pub fn boom(&mut self) {
        self.exposed_animation.go_mut(true, self.now);
    }

    pub fn view(&self) -> Element<'_, AppMsg> {
        let adjacent_mines = self.cell.adjacent_mines;

        let content: Element<'_, AppMsg> = match self.cell.state {
            CellState::Open => container(if self.cell.adjacent_mines > 0 {
                text(format!("{adjacent_mines}"))
                    .center()
                    .style(move |_| {
                        if self.cell_animation.is_animating(self.now) {
                            select_color(
                                adjacent_mines,
                                self.cell_animation.interpolate(0.0, 1.0, self.now),
                            )
                        } else {
                            select_color(adjacent_mines, 1.0)
                        }
                    })
                    .center()
            } else {
                text("")
            })
            .center(Length::Fill)
            .style(|theme| container::primary(theme).background(Color::WHITE))
            .into(),

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
                                            color.a = if self.cell_animation.is_animating(self.now)
                                            {
                                                self.cell_animation.interpolate(0.0, 1.0, self.now)
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
                                style.text_color.a = if self.cell_animation.is_animating(self.now) {
                                    self.cell_animation.interpolate(0.0, 1.0, self.now)
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
                    .style(|theme| {
                        let animated_opacity_color = |mut color: Color| {
                            color.a = if self.exposed_animation.is_animating(self.now) {
                                self.exposed_animation.interpolate(0.0, 1.0, self.now)
                            } else {
                                1.0
                            };
                            color
                        };
                        container::primary(theme)
                            .color(animated_opacity_color(color!(0xf9f06b)))
                            .background(animated_opacity_color(color!(0xa51d2d)))
                    })
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
                1 => Color::BLACK,
                2 => color!(0x26a269),
                3 => color!(0xa51d2d),
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
