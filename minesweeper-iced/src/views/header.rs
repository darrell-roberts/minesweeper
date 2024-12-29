use super::format_elapsed;
use iced::{
    widget::{container, row, text, Column},
    Element, Length,
};
use minesweeper::model::Board;

pub struct Header {
    elapsed_seconds: u64,
    opened: usize,
    flagged: usize,
    mined: usize,
}

impl Header {
    pub fn new(board: &Board, elapsed_seconds: u64) -> Self {
        Self {
            elapsed_seconds,
            opened: board.opened(),
            flagged: board.flagged(),
            mined: board.mined(),
        }
    }
}

impl Header {
    pub fn view<'a, Message>(&self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let column = Column::new().push(
            container(
                row![
                    text(format!("Opened: {}", self.opened)),
                    text(format!("Flagged: {}", self.flagged)),
                    text(format!("Mined: {}", self.mined)),
                    text(format!("Time: {}", format_elapsed(self.elapsed_seconds)))
                ]
                .spacing(20),
            )
            .width(Length::Fill)
            .center_x(Length::Fill),
        );

        container(column)
            .padding(10)
            .width(Length::Fill)
            .center_x(Length::Fill)
            .into()
    }
}
