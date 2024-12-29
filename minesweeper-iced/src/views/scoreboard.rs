use super::format_elapsed;
use iced::{
    widget::{container, row, text, Column},
    Color, Element,
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
        let col = self.win_history.iter().zip(1..).fold(
            Column::new(),
            |col, (win, rank)| {
                let row = row![
                    container(text(format!("{rank:<5}"))).width(20),
                    container(text(format_elapsed(win.duration))).width(200),
                    text(format!("{}", win.date.format("%b %d %Y %I:%M%P")))
                ]
                .spacing(10);
                col.push(row).spacing(10)
            },
        );
        container(col)
            .padding(20)
            .style(|_| {
                container::Style::default()
                    .background(Color::from_rgb8(0, 153, 204))
            })
            .into()
    }
}
