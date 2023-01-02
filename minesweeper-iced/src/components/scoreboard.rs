use super::format_elapsed;
use iced::{
    widget::{component, container, row, text, Column, Component},
    Element, Renderer,
};
use minesweeper::history::Win;

pub struct ScoreBoard<'a> {
    win_history: &'a [Win],
}

impl<'a> ScoreBoard<'a> {
    pub fn new(win_history: &'a [Win]) -> Self {
        Self { win_history }
    }
}

impl<'a, Message> Component<Message, Renderer> for ScoreBoard<'a> {
    type State = ();
    type Event = ();

    fn update(
        &mut self,
        _state: &mut Self::State,
        _event: Self::Event,
    ) -> Option<Message> {
        None
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event, Renderer> {
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
        container(col).padding(20).into()
    }
}

impl<'a, Message> From<ScoreBoard<'a>> for Element<'a, Message, Renderer>
where
    Message: 'a,
{
    fn from(value: ScoreBoard<'a>) -> Self {
        component(value)
    }
}
