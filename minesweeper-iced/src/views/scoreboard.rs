use super::format_elapsed;
use iced::{
    widget::{container, row, text, Column},
    Element,
};
use minesweeper::history::Win;

pub struct ScoreBoard<'a> {
    win_history: &'a [Win],
}

impl<'a> ScoreBoard<'a> {
    pub fn new(win_history: &'a [Win]) -> Self {
        Self { win_history }
    }

    pub fn view<Message>(&self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let col = self
            .win_history
            .iter()
            .zip(1..)
            .fold(Column::new(), |col, (win, rank)| {
                let row = row![
                    container(text(format!("{rank:<5}")).size(20)).width(25),
                    container(text(format_elapsed(win.duration)).size(20)).width(250),
                    text(format!("{}", win.date.format("%b %d %Y %I:%M%P"))).size(20)
                ]
                .spacing(10);
                col.push(row).spacing(10)
            });
        container(col).padding(20).into()
    }
}
