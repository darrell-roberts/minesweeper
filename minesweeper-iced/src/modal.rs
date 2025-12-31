//! Modal helper.
use iced::{
    Color, Element, Length, padding,
    widget::{container, mouse_area, opaque, stack},
};

pub fn modal<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
    on_blur: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    stack![
        base.into(),
        opaque(
            mouse_area(
                container(opaque(content))
                    .center(Length::Fill)
                    .padding(padding::top(10))
                    .style(move |_theme| {
                        container::Style {
                            background: Some(
                                Color {
                                    a: 0.5,
                                    ..Color::BLACK
                                }
                                .into(),
                            ),
                            ..container::Style::default()
                        }
                    })
            )
            .on_press(on_blur)
        )
    ]
    .into()
}
