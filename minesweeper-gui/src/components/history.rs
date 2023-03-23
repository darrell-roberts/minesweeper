use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use relm4::{
  factory::FactoryVecDeque, gtk, gtk::prelude::*, prelude::FactoryComponent,
  ComponentParts, SimpleComponent,
};
use rmp_serde::{encode::write_named, from_read};
use serde::{Deserialize, Serialize};
use std::{
  fs::{create_dir_all, OpenOptions},
  io::{BufWriter, Seek},
};

use crate::format_elapsed;

#[derive(Serialize, Deserialize, Debug)]
pub struct Win {
  date: DateTime<Local>,
  duration: u64,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct WinHistory {
  wins: Vec<Win>,
}

#[derive(Debug)]
pub struct WinHistoryView {
  win_history: FactoryVecDeque<Win>,
  hidden: bool,
}

#[cfg(test)]
const SAVE_FILE_PATH: &str = "/tmp/minesweeper/";

#[cfg(not(test))]
const SAVE_FILE_PATH: &str = ".local/share/minesweeper/";

#[cfg(not(test))]
fn get_save_file() -> Result<String> {
  std::env::var("HOME")
    .with_context(|| "Could not lookup $HOME environment variable")
    .map(|home| [&home, "/", SAVE_FILE_PATH, SAVE_FILE].concat())
    .map(Ok)?
}

#[cfg(test)]
fn get_save_file() -> Result<String> {
  Ok([SAVE_FILE_PATH, SAVE_FILE].concat())
}

#[cfg(not(test))]
fn get_full_save_path() -> Result<String> {
  std::env::var("HOME")
    .with_context(|| "Could not lookup up $HOME environment variable")
    .map(|home| [&home, "/", SAVE_FILE_PATH].concat())
    .map(Ok)?
}

#[cfg(test)]
fn get_full_save_path() -> Result<String> {
  Ok(SAVE_FILE_PATH.into())
}

const SAVE_FILE: &str = "stats.bin";

/// Save the win to the win history.
pub fn save_win(duration: u64) -> Result<()> {
  persist_win(Win {
    duration,
    date: Local::now(),
  })
}

/// Load win history.
fn load_wins() -> Option<WinHistory> {
  OpenOptions::new()
    .read(true)
    .open(get_save_file().ok()?)
    .ok()
    .and_then(|stats_file| from_read(&stats_file).ok())
}

// Save the best 10 results as a MessagePack format.
fn persist_win(win: Win) -> anyhow::Result<()> {
  create_dir_all(get_full_save_path()?)
    .with_context(|| "Could not create folder for stats file")?;
  let mut stats_file = OpenOptions::new()
    .write(true)
    .read(true)
    .create(true)
    .open(get_save_file()?)?;
  let mut history: WinHistory = from_read(&stats_file).unwrap_or_else(|err| {
    eprintln!("Failed to read stats file: {err}. Creating new WinHistory");
    WinHistory::default()
  });
  history.wins.push(win);
  history.wins.sort_by_key(|win| win.duration);
  history.wins = history.wins.into_iter().take(10).collect();
  stats_file.rewind()?;
  let mut writer = BufWriter::new(stats_file);
  write_named(&mut writer, &history)?;
  Ok(())
}

#[derive(Debug)]
pub enum HistoryMsg {
  Open,
  Close,
  Reload,
}

#[derive(Debug)]
pub enum HistoryOut {
  Resume,
}

#[relm4::component(pub)]
impl SimpleComponent for WinHistoryView {
  type Input = HistoryMsg;
  type Output = HistoryOut;
  type Init = ();

  view! {
      gtk::Window {
          set_modal: true,
          set_default_width: 400,
          set_default_height: 400,
          #[watch]
          set_visible: !model.hidden,
          set_deletable: false,
          set_decorated: false,

          #[wrap(Some)]
          set_child = &gtk::Box {
              set_css_classes: &["winHistoryWindow"],
              set_orientation: gtk::Orientation::Vertical,

              gtk::Label {
                  set_label: "Top 10 Scores",
                  set_css_classes: &["winHistoryHeader"],
              },

              #[local_ref]
              win_box -> gtk::Box {
                  set_vexpand: true,
                  set_orientation: gtk::Orientation::Vertical,
              },
              gtk::Button {
                  set_css_classes: &["winHistoryButton"],
                  set_label: "Close",
                  connect_clicked => HistoryMsg::Close,
              }
          },
      }
  }

  fn init(
    _init: Self::Init,
    root: &Self::Root,
    sender: relm4::ComponentSender<Self>,
  ) -> relm4::ComponentParts<Self> {
    let wins = load_wins().map(|w| {
      FactoryVecDeque::from_vec(
        w.wins,
        gtk::Box::default(),
        sender.input_sender(),
      )
    });

    let win_history = wins.unwrap_or_else(|| {
      FactoryVecDeque::new(gtk::Box::default(), sender.input_sender())
    });
    let model = WinHistoryView {
      hidden: true,
      win_history,
    };
    let win_box = model.win_history.widget();
    let widgets = view_output!();

    ComponentParts { model, widgets }
  }

  fn update(
    &mut self,
    message: Self::Input,
    sender: relm4::ComponentSender<Self>,
  ) {
    match message {
      HistoryMsg::Open => {
        self.hidden = false;
      }
      HistoryMsg::Close => {
        self.hidden = true;
        sender.output_sender().emit(HistoryOut::Resume);
      }
      HistoryMsg::Reload => {
        self.win_history.guard().clear();
        for win in load_wins().into_iter().flat_map(|w| w.wins.into_iter()) {
          self.win_history.guard().push_back(win);
        }
      }
    }
  }
}

#[relm4::factory(pub)]
impl FactoryComponent for Win {
  type Init = Win;
  type Input = ();
  type Output = ();
  type CommandOutput = ();
  type Widgets = WinWidgets;
  type ParentInput = HistoryMsg;
  type ParentWidget = gtk::Box;

  view! {
      root = gtk::Box {
          set_orientation: gtk::Orientation::Horizontal,
          set_spacing: 10,
          set_halign: gtk::Align::Center,

          gtk::Box {
              set_halign: gtk::Align::End,
              // set_width_request: 20,
              gtk::Label {
                  set_css_classes: &["winHistoryRank"],
                  set_label: &format!("{}.", index.current_index() + 1)
              },
          },

          gtk::Box {
              set_spacing: 10,
              set_css_classes: &["winHistory"],
              set_orientation: gtk::Orientation::Horizontal,
              gtk::Label {
                  set_label: &format_elapsed(self.duration)
              },
              gtk::Label {
                  set_label: &format!("{}", self.date.format("%b %e / %G %R"))
              }
          }
      }
  }

  fn init_model(
    init: Self::Init,
    _index: &relm4::prelude::DynamicIndex,
    _sender: relm4::FactorySender<Self>,
  ) -> Self {
    init
  }
}
