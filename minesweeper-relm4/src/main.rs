use minesweeper_relm4::{board, AppModel};
use relm4::RelmApp;

fn main() {
    let relm = RelmApp::new("dr.minesweeper");
    relm.set_global_css(include_str!("style.css"));
    relm.run::<AppModel>(board());
}
