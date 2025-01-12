use crate::types::Position;
use minesweeper::model::{Cell, CellState, Pos};
use relm4::{
    factory::{positions::GridPosition, FactoryComponent, Position as FactoryPosition},
    gtk,
    gtk::prelude::*,
    prelude::DynamicIndex,
};

#[derive(Debug)]
pub struct FactoryWidgets {
    container: gtk::Box,
    button: gtk::Button,
    _gesture: gtk::GestureClick,
}

#[derive(Debug)]
pub enum PositionOutput {
    Open(Pos),
    Flag(Position),
}

// static EMPTY: &str = "      ";
static EMPTY: &str = "";

impl FactoryComponent for Position {
    type Init = (Pos, Cell);
    type Input = ();
    type Output = PositionOutput;
    type CommandOutput = ();
    type Widgets = FactoryWidgets;
    type ParentWidget = gtk::Grid;
    type Root = gtk::Box;
    type Index = DynamicIndex;

    fn init_model(
        (pos, cell): Self::Init,
        dyn_index: &relm4::prelude::DynamicIndex,
        _sender: relm4::FactorySender<Self>,
    ) -> Self {
        Self {
            index: dyn_index.current_index(),
            pos,
            cell,
        }
    }

    fn init_root(&self) -> Self::Root {
        gtk::Box::default()
    }

    fn init_widgets(
        &mut self,
        _index: &relm4::prelude::DynamicIndex,
        root: Self::Root,
        _returned_widget: &<Self::ParentWidget as relm4::factory::FactoryView>::ReturnedWidget,
        sender: relm4::FactorySender<Self>,
    ) -> Self::Widgets {
        let mut container = gtk::Box::builder().css_name("container");
        let mut button = gtk::Button::builder().label(EMPTY);
        match self.cell.state {
            CellState::Open => {
                button = button.css_classes(vec![
                    "cell",
                    "open",
                    adjacent_mine_style(*self).unwrap_or_default(),
                ]);
                container = container.css_classes(vec!["open"]);
                if self.cell.adjacent_mines > 0 {
                    button = button.label(adjacent_mine_label(*self));
                }
            }
            CellState::Closed { flagged, .. } => {
                if flagged {
                    button = button.css_classes(vec!["cell", "flagged"]).label("F");
                    container = container.css_classes(vec!["flagged"]);
                } else {
                    button = button.css_classes(vec!["cell", "closed"]);
                    container = container.css_classes(vec!["closed"]);
                }
            }
            CellState::ExposedMine => {
                button = button.css_classes(vec!["cell", "exposed"]).label("X");
            }
        }

        let button = button.build();
        let container = container.build();
        {
            let pos_selected = self.pos;
            let sender = sender.clone();
            button.connect_clicked(move |_| {
                if let Err(err) = sender.output(PositionOutput::Open(pos_selected)) {
                    eprintln!("Failed to send open cell {err:?}");
                }
            });
        }
        let right_click = gtk::GestureClick::builder().button(3).build();
        {
            let pos_selected = *self;
            right_click.connect_pressed(move |gesture, _, _, _| {
                gesture.set_state(gtk::EventSequenceState::Claimed);
                if let Err(err) = sender.output(PositionOutput::Flag(pos_selected)) {
                    eprintln!("Failed to send flag cell {err:?}");
                };
            });
        }
        container.append(&button);
        container.add_controller(right_click.clone());

        root.append(&container);

        FactoryWidgets {
            container,
            button,
            _gesture: right_click,
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, _sender: relm4::FactorySender<Self>) {
        let label = match self.cell.state {
            CellState::Open => {
                widgets.button.set_css_classes(&[
                    "cell",
                    "open",
                    adjacent_mine_style(*self).unwrap_or_default(),
                ]);
                widgets.container.set_css_classes(&["open"]);
                if self.cell.adjacent_mines > 0 {
                    adjacent_mine_label(*self)
                } else {
                    EMPTY
                }
            }
            CellState::Closed { flagged, .. } => {
                if flagged {
                    widgets.button.set_css_classes(&["cell", "flagged"]);
                    widgets.container.set_css_classes(&["flagged"]);
                    "F"
                } else {
                    widgets.button.set_css_classes(&["cell", "closed"]);
                    widgets.container.set_css_classes(&["closed"]);
                    EMPTY
                }
            }
            CellState::ExposedMine => {
                widgets.button.set_css_classes(&["cell", "exposed"]);
                widgets.container.set_css_classes(&["exposed"]);
                "B"
            }
        };
        widgets.button.set_label(label);
    }
}

impl FactoryPosition<GridPosition, DynamicIndex> for Position {
    fn position(&self, _index: &DynamicIndex) -> GridPosition {
        GridPosition {
            column: self.pos.x.get() as i32,
            row: self.pos.y.get() as i32,
            width: 1,
            height: 1,
        }
    }
}

fn adjacent_mine_label(pos: Position) -> &'static str {
    match pos.cell.adjacent_mines {
        1 => "1",
        2 => "2",
        3 => "3",
        4 => "4",
        5 => "5",
        6 => "6",
        7 => "7",
        8 => "8",
        _ => unreachable!(),
    }
}

fn adjacent_mine_style(pos: Position) -> Option<&'static str> {
    match pos.cell.adjacent_mines {
        0 => None?,
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "four",
    }
    .into()
}
