use minesweeper_gui::{board, AppModel};
use relm4::RelmApp;

fn main() {
  let relm = RelmApp::new("dr.minesweeper");
  relm4::set_global_css(include_str!("style.css"));
  relm.run::<AppModel>(board());
}
