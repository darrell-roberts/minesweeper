use minesweeper_relm4::{board, AppModel};
use relm4::RelmApp;

fn main() {
    let relm = RelmApp::new("io.github.darrellroberts-gtk4");
    relm4::set_global_css(include_str!("style.css"));
    relm.run::<AppModel>(board());
}
