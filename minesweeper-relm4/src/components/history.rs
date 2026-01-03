use crate::format_elapsed;
use chrono::{DateTime, Local};
use minesweeper::history::{load_wins, Win};
use relm4::{
    factory::FactoryVecDeque, gtk, gtk::prelude::*, prelude::FactoryComponent, ComponentParts,
    SimpleComponent,
};

#[derive(Debug)]
pub struct WinHistoryView {
    win_history: FactoryVecDeque<WinData>,
    hidden: bool,
}

#[derive(Debug)]
pub struct WinData(Win);

impl WinData {
    fn date(&self) -> DateTime<Local> {
        self.0.date
    }

    fn duration(&self) -> u64 {
        self.0.duration
    }
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

                // #[local_ref]
                gtk::Box {
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
        root: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let wins = load_wins().map(|w| {
            FactoryVecDeque::from_iter(w.wins.into_iter().map(WinData), gtk::Box::default())
        });

        let win_history = wins
            .unwrap_or_else(|| FactoryVecDeque::from_iter(std::iter::empty(), gtk::Box::default()));
        let model = WinHistoryView {
            hidden: true,
            win_history,
        };
        let _win_box = model.win_history.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: relm4::ComponentSender<Self>) {
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
                    self.win_history.guard().push_back(WinData(win));
                }
            }
        }
    }
}

#[relm4::factory(pub)]
impl FactoryComponent for WinData {
    type Init = WinData;
    type Input = ();
    type Output = ();
    type CommandOutput = ();
    type Widgets = WinWidgets;
    type ParentWidget = gtk::Box;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,

            gtk::Box {
                set_valign: gtk::Align::Center,
                set_halign: gtk::Align::End,
                set_orientation: gtk::Orientation::Vertical,
                set_css_classes: &["winHistoryRank"],
                gtk::Label {
                    set_halign: gtk::Align::End,
                    set_label: &format!("{}.", index.current_index() + 1)
                },
            },

            gtk::Box {
                set_spacing: 10,
                set_css_classes: &["winHistory"],
                set_orientation: gtk::Orientation::Horizontal,
                set_hexpand: true,
                gtk::Label {
                    set_label: &format_elapsed(self.duration())
                },
                gtk::Label {
                    set_label: &format!("{}", self.date().format("%b %e / %G %R"))
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
