//! Modal helper.
use iced::{
    Color, Element, Length, padding,
    widget::{container, mouse_area, opaque, stack},
};

/// Create a modal using content over the base elements.
pub fn modal<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
    on_blur: Message,
) -> impl Into<Element<'a, Message>>
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
                    .style(move |theme| {
                        let palette = theme.extended_palette();
                        container::Style {
                            background: Some(
                                Color {
                                    a: 0.5,
                                    ..palette.primary.strong.color
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
}
