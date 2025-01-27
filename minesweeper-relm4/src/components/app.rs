use super::{
    history::{HistoryMsg, HistoryOut, WinHistoryView},
    status_dialog::{StatusDialogModel, StatusMsg},
    timer::{GameTimer, GameTimerInput, GameTimerOutput},
};
use crate::{
    board, components::positions::PositionOutput, format_elapsed, types::Position, BOMB, FLAG,
};
use minesweeper::{
    history::save_win,
    model::{Board, GameState, Pos},
};
use relm4::{
    factory::FactoryVecDeque, gtk, gtk::prelude::*, Component, ComponentController, ComponentParts,
    ComponentSender, Controller, SimpleComponent, WorkerController,
};
use std::collections::HashMap;

/// Application state.
pub struct AppModel {
    /// Game board and API
    board: Board,
    /// View model for board.
    positions: FactoryVecDeque<Position>,
    /// Map Pos items to index in [FactoryVec].
    pos_map: HashMap<Pos, usize>,
    /// Status dialog.
    dialog: Controller<StatusDialogModel>,
    /// Background working tracking game time.
    timer_worker: WorkerController<GameTimer>,
    /// The elapsed time from game start to end.
    time_elapsed: u64,
    /// Elapsed time copied when paused.
    time_paused: u64,
    /// If the game is paused.
    paused: bool,
    /// History view window.
    history_window: Controller<WinHistoryView>,
}

impl AppModel {
    /// Sync up the view model with the game board.
    fn update_all_positions(&mut self) {
        self.positions.guard().clear();
        for (&pos, &cell) in self.board.positions() {
            self.positions.guard().push_back((pos, cell));
        }
    }

    /// Replace View positions cells with updated board cell.
    fn update_positions(&mut self, positions: &[Position]) {
        for p in positions {
            if let Some(pos) = self.positions.guard().get_mut(p.index) {
                *pos = *p;
            }
        }
    }
}

/// User actions
#[derive(Debug)]
pub enum AppMsg {
    /// Open a position on the board.
    Open(Pos),
    /// Flag a position on the board.
    Flag(Position),
    /// Start a new game, resetting the board.
    Start,
    /// Timer tick.
    Tick(u64),
    /// Show win history.
    ShowHistory,
    /// Resume an active game.
    Resume,
}

#[relm4::component(pub)]
impl SimpleComponent for AppModel {
    type Input = AppMsg;
    type Output = ();
    type Init = Board;

    view! {
        gtk::Window {
          set_title: Some("Minesweeper"),
          set_default_width: 250,
          set_default_height: 100,
          gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 5,

            #[name = "header"]
            gtk::Box {
              set_halign: gtk::Align::Center,
              set_orientation: gtk::Orientation::Horizontal,
              set_spacing: 25,
              set_css_classes: &["header"],

              gtk::Box {
                gtk::Label {
                  set_label: "Time: ",
                },
                #[name = "time_label"]
                gtk::Label {
                  #[watch]
                  set_label: &format_elapsed(model.time_elapsed),
                  set_css_classes: &["time"],
                  set_halign: gtk::Align::Start,
                }
              },

              gtk::Box {
                gtk::Label {
                  set_label: "Opened: ",
                },
                #[name = "opened"]
                gtk::Label {
                  #[watch]
                  set_label: &format!("{}", model.board.opened()),
                }
              },

              gtk::Box {
                gtk::Label {
                  set_label: &format!("{FLAG}: "),
                },
                #[name = "flagged"]
                gtk::Label {
                  #[watch]
                  set_label: &format!("{}", model.board.flagged()),
                }
              },

              gtk::Box {
                gtk::Label {
                  set_label: &format!("{BOMB}: "),
                },
                #[name = "mined"]
                gtk::Label {
                  #[watch]
                  set_label:&format!("{}", model.board.mined()),
                }
              },
            },

            #[local_ref]
            factory_board -> gtk::Grid {
              set_orientation: gtk::Orientation::Vertical,
              set_column_spacing: 5,
              set_row_spacing: 5,
              set_halign: gtk::Align::Fill,
              set_hexpand: true,
              set_valign: gtk::Align::Fill,
              set_vexpand: true,
              #[watch]
              set_sensitive: !matches!(model.board.state(), GameState::Win | GameState::Loss),
              #[watch]
              set_opacity: if model.paused {
                  0.1
              } else {
                  1.
              },
            },

            gtk::Box {
                set_halign: gtk::Align::Center,
                set_spacing: 10,
                gtk::Button {
                  set_label: "Top Scores",
                  set_css_classes: &["button"],
                  connect_clicked => AppMsg::ShowHistory
                },

                gtk::Button {
                    set_label: "Restart",
                    set_css_classes: &["button"],
                    connect_clicked => AppMsg::Start
                }
            }
          },
      }
    }

    fn init(
        board: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut positions = FactoryVecDeque::builder()
            .launch(gtk::Grid::default())
            .forward(sender.input_sender(), |output| match output {
                PositionOutput::Open(p) => AppMsg::Open(p),
                PositionOutput::Flag(p) => AppMsg::Flag(p),
            });

        for (&pos, &cell) in board.positions() {
            positions.guard().push_back((pos, cell));
        }

        let pos_map = positions
            .iter()
            .map(|Position { index, pos, .. }| (*pos, *index))
            .collect::<HashMap<_, _>>();

        let model = AppModel {
            board,
            pos_map,
            positions,
            dialog: StatusDialogModel::builder()
                .transient_for(&root)
                .launch(true)
                .detach(),
            timer_worker: GameTimer::builder().detach_worker(()).forward(
                sender.input_sender(),
                |msg| match msg {
                    GameTimerOutput::Tick(n) => AppMsg::Tick(n),
                },
            ),
            time_elapsed: 0,
            time_paused: 0,
            paused: false,
            history_window: WinHistoryView::builder()
                .transient_for(&root)
                .launch(())
                .forward(sender.input_sender(), |msg| match msg {
                    HistoryOut::Resume => AppMsg::Resume,
                }),
        };

        let factory_board = model.positions.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::Open(p) => {
                if self.board.state() == &GameState::New {
                    self.timer_worker
                        .sender()
                        .send(GameTimerInput::Start)
                        .unwrap();
                }
                let opened = self.board.open_cell(p);

                match *self.board.state() {
                    s @ GameState::Loss | s @ GameState::Win => {
                        if s == GameState::Win {
                            save_win(self.time_elapsed)
                                .unwrap_or_else(|e| eprintln!("Failed to save game win {e}"));
                            self.history_window.emit(HistoryMsg::Reload);
                        }
                        self.update_all_positions();
                        self.timer_worker
                            .sender()
                            .send(GameTimerInput::Stop)
                            .unwrap_or_else(|_| eprintln!("Failed to stop timer"));
                        self.dialog
                            .sender()
                            .send(StatusMsg::Open(if s == GameState::Win {
                                "You win!".into()
                            } else {
                                "You lose!".into()
                            }))
                            .unwrap_or_else(|_| eprintln!("Failed to send message"));
                    }
                    _ => {
                        let matched_pos = opened
                            .into_iter()
                            .flat_map(|(pos, cell)| {
                                self.pos_map
                                    .get(&pos)
                                    .map(|&index| Position { pos, cell, index })
                            })
                            .collect::<Vec<_>>();
                        self.update_positions(&matched_pos);
                    }
                }
            }
            AppMsg::Flag(p) => {
                if let Some(position) = self.board.flag_cell(p.pos).and_then(|(pos, cell)| {
                    self.pos_map
                        .get(&pos)
                        .map(|&index| Position { pos, cell, index })
                }) {
                    self.update_positions(&[position]);
                }
            }
            AppMsg::Start => {
                self.timer_worker.emit(GameTimerInput::Stop);
                self.board = board();
                self.update_all_positions();
                self.time_elapsed = 0;
                self.time_paused = 0;
                self.paused = false;
            }
            AppMsg::Tick(seconds) => {
                if !self.paused && *self.board.state() == GameState::Active {
                    self.time_elapsed = seconds + self.time_paused;
                }
            }
            AppMsg::ShowHistory => {
                if *self.board.state() == GameState::Active {
                    self.paused = true;
                    self.timer_worker.emit(GameTimerInput::Stop);
                    self.time_paused = self.time_elapsed;
                }
                self.history_window.emit(super::history::HistoryMsg::Open);
            }
            AppMsg::Resume => {
                if self.paused && *self.board.state() == GameState::Active {
                    self.timer_worker.emit(GameTimerInput::Start);
                    self.paused = false;
                }
            }
        }
    }
}
