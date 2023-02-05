use crate::{types::Position, AppModel, AppMsg};
use minesweeper::model::{CellState, GameState};
use relm4::{
  factory::{positions::GridPosition, FactoryPrototype, FactoryVec},
  gtk::prelude::*,
  send, Widgets,
};

#[relm4_macros::widget(pub)]
impl Widgets<AppModel, ()> for AppWidgets {
  view! {
      gtk::ApplicationWindow {
        set_title: Some("Minesweeper"),
        set_default_width: 300,
        set_default_height: 100,
        set_child = container = Some(&gtk::Box) {
          set_orientation: gtk::Orientation::Vertical,
          set_spacing: 5,

          append = header = &gtk::Box {
            set_halign: gtk::Align::Center,
            set_orientation: gtk::Orientation::Horizontal,
            set_spacing: 250,
            set_margin_bottom: 10,
            set_margin_top: 10,

            append = &gtk::Box {
              append = &gtk::Label {
                set_label: "Opened: ",
              },
              append = opened = &gtk::Label {
                set_label: watch! { &format!("{}", model.board.opened()) },
              }
            },

            append = &gtk::Box {
              append = &gtk::Label {
                set_label: "Flagged: ",
              },
              append = flagged = &gtk::Label {
                set_label: watch! { &format!("{}", model.board.flagged()) },
              }
            },

            append = &gtk::Box {
              append = &gtk::Label {
                set_label: "Mined: ",
              },
              append = mined = &gtk::Label {
                set_label: watch! { &format!("{}", model.board.total_mines()) },
              }
            },
          },

          append = board = &gtk::Grid {
            set_orientation: gtk::Orientation::Vertical,
            set_column_spacing: 5,
            set_row_spacing: 5,
            set_halign: gtk::Align::Fill,
            set_hexpand: true,
            set_valign: gtk::Align::Fill,
            set_vexpand: true,
            set_sensitive: watch! {
              match model.board.state() {
                GameState::Win | GameState::Loss => {
                  false
                },
                GameState::New | GameState::Active => {
                  true
                }
              }
            },
            factory!(model.positions)
          },

          append = &gtk::Button {
            set_halign: gtk::Align::Center,
            set_label: "restart",
            set_css_classes: &["restart"],
            connect_clicked(sender) => move |_| {
              send!(sender, AppMsg::Start)
            }
          }
      },
    }
  }
}

#[derive(Debug)]
pub struct FactoryWidgets {
  container: gtk::Box,
  button: gtk::Button,
  _gesture: gtk::GestureClick,
}

static EMPTY: &str = "      ";

impl FactoryPrototype for Position {
  type Factory = FactoryVec<Self>;
  type Widgets = FactoryWidgets;
  type Root = gtk::Box;
  type View = gtk::Grid;
  type Msg = AppMsg;

  fn root_widget(widgets: &Self::Widgets) -> &Self::Root {
    &widgets.container
  }

  fn init_view(
    &self,
    _key: &<Self::Factory as relm4::factory::Factory<Self, Self::View>>::Key,
    sender: relm4::Sender<Self::Msg>,
  ) -> Self::Widgets {
    let mut container = gtk::Box::builder().spacing(10).css_name("container");
    let mut button = gtk::Button::builder().label(EMPTY);
    match self.cell.state {
      CellState::Open => {
        button = button.css_classes(vec!["cell".into(), "open".into()]);
        container = container.css_classes(vec!["open".into()]);
        if self.cell.adjacent_mines > 0 {
          button = button.label(adjacent_mine_label(*self));
        }
      }
      CellState::Closed { flagged, .. } => {
        if flagged {
          button = button
            .css_classes(vec!["cell".into(), "flagged".into()])
            .label("🚩");
          container = container.css_classes(vec!["flagged".into()]);
        } else {
          button = button.css_classes(vec!["cell".into(), "closed".into()]);
          container = container.css_classes(vec!["closed".into()]);
        }
      }
      CellState::ExposedMine => {
        button = button
          .css_classes(vec!["cell".into(), "exposed".into()])
          .label("💣");
      }
    }

    let button = button.build();
    let container = container.build();
    {
      let pos_selected = self.pos;
      let sender = sender.clone();
      button.connect_clicked(move |_| {
        send!(sender, AppMsg::Open(pos_selected));
      });
    }
    let right_click = gtk::GestureClick::builder().button(3).build();
    {
      let pos_selected = *self;
      right_click.connect_pressed(move |gesture, _, _, _| {
        gesture.set_state(gtk::EventSequenceState::Claimed);
        send!(sender, AppMsg::Flag(pos_selected));
      });
    }
    container.append(&button);
    container.add_controller(&right_click);

    FactoryWidgets {
      container,
      button,
      _gesture: right_click,
    }
  }

  fn view(
    &self,
    _key: &<Self::Factory as relm4::factory::Factory<Self, Self::View>>::Key,
    widgets: &Self::Widgets,
  ) {
    // println!("Updating key {key}");
    let label = match self.cell.state {
      CellState::Open => {
        widgets.button.set_css_classes(&["cell", "open"]);
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
          "🚩"
        } else {
          widgets.button.set_css_classes(&["cell", "closed"]);
          widgets.container.set_css_classes(&["closed"]);
          EMPTY
        }
      }
      CellState::ExposedMine => {
        widgets.button.set_css_classes(&["cell", "exposed"]);
        widgets.container.set_css_classes(&["exposed"]);
        "💣"
      }
    };
    widgets.button.set_label(label);
  }

  fn position(
    &self,
    _key: &<Self::Factory as relm4::factory::Factory<Self, Self::View>>::Key,
  ) -> <Self::View as relm4::factory::FactoryView<Self::Root>>::Position {
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
