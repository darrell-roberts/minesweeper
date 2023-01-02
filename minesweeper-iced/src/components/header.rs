use iced::{
    widget::{component, container, row, text, Column, Component},
    Element, Length, Renderer,
};
use minesweeper::model::Board;

use super::format_elapsed;

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

impl<Message> Component<Message, Renderer> for Header {
    type State = ();
    type Event = ();

    fn update(
        &mut self,
        _state: &mut Self::State,
        _event: Self::Event,
    ) -> Option<Message> {
        unreachable!()
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event, Renderer> {
        let mut column = Column::default();

        column = column.push(
            container(
                row![
                    text(format!("Opened: {}", self.opened)),
                    text(format!("Flagged: {}", self.flagged)),
                    text(format!("Mined: {}", self.mined)),
                    text(format!(
                        "Time: {}",
                        format_elapsed(self.elapsed_seconds)
                    ))
                ]
                .spacing(20),
            )
            .width(Length::Fill)
            .center_x(),
        );

        container(column)
            .padding(10)
            .width(Length::Fill)
            .center_x()
            .into()
    }
}

impl<'a, Message> From<Header> for Element<'a, Message, Renderer>
where
    Message: 'a,
{
    fn from(value: Header) -> Self {
        component(value)
    }
}
