use iced::{
    widget::{component, container, row, text, Column, Component},
    Element, Length, Renderer,
};
use minesweeper::model::{Board, GameState};

pub struct Header {
    elapsed_seconds: u64,
    opened: usize,
    flagged: usize,
    mined: usize,
    game_state: GameState,
}

impl Header {
    pub fn new(board: &Board, elapsed_seconds: u64) -> Self {
        Self {
            elapsed_seconds,
            opened: board.opened(),
            flagged: board.flagged(),
            mined: board.mined(),
            game_state: *board.state(),
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
        let centered = |text| container(text).width(Length::Fill).center_x();

        let mut column = Column::default();
        if self.game_state == GameState::Win {
            column = column.push(centered(text("You Win!")));
        } else if self.game_state == GameState::Loss {
            column = column.push(centered(text("You Lose!")));
        }

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

/// Displayable elapsed time.
fn format_elapsed(seconds: u64) -> String {
    match seconds {
        0..=59 => format!("{seconds} seconds"),
        60..=3599 => format!(
            "{} minute(s) {} seconds",
            seconds.div_euclid(60),
            seconds.rem_euclid(60)
        ),
        3600.. => format!("{} hours", seconds.div_euclid(3600)),
    }
}
